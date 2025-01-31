use std::str::FromStr;

use bigdecimal::BigDecimal;
use connector_model::{orderbook::{OrderBook, PriceLevel}, pricing::{Quantity, Rate}};
use serde::Deserialize;

use crate::errors::data_fetching_errors::DataFetchingErrors;

#[derive(Deserialize, Debug)]
pub(super) struct DeepBookV3OrderBookData {
    timestamp: String,
    bids: Vec<DeepBookV3PriceLevel>,
    asks: Vec<DeepBookV3PriceLevel>
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "Vec<String>")]
struct DeepBookV3PriceLevel {
    price: BigDecimal,
    quantity: BigDecimal
}

impl TryFrom<Vec<String>> for DeepBookV3PriceLevel {
    type Error = DataFetchingErrors;
    fn try_from(value: Vec<String>) -> std::result::Result<Self, Self::Error> {
        if value.len() == 2 {
            Ok(DeepBookV3PriceLevel {
                price: BigDecimal::from_str(&value[0])?,
                quantity: BigDecimal::from_str(&value[1])?
            })
        } else {
            Err(DataFetchingErrors::DeepBookV3DeserialisationError())
        } 
    }
}

impl From<DeepBookV3PriceLevel> for PriceLevel {
    fn from(value: DeepBookV3PriceLevel) -> Self {
        PriceLevel {
            price: Rate::new(value.price),
            quantity: Quantity::new(value.quantity)
        }
    }
}

impl From<DeepBookV3OrderBookData> for OrderBook {
    fn from(value: DeepBookV3OrderBookData) -> Self {

        let ask_price_levels = value.asks
        .into_iter()
        .map(| data | PriceLevel::from(data))
        .collect();

        let bid_price_levels= value.bids.into_iter().
        map(|data| PriceLevel::from(data))
        .collect();

        OrderBook::new(bid_price_levels, ask_price_levels)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn convert_deepbook_order_book_to_generic_order_book(){

        let input_bids: Vec<String> = vec!["2".to_string(), "3".to_string()];
        let input_asks: Vec<String> = vec!["1".to_string(), "2".to_string()];

        let input_db_bids = DeepBookV3PriceLevel::try_from(input_bids).unwrap();
        let input_db_asks = DeepBookV3PriceLevel::try_from(input_asks).unwrap();

        let test_deep_book_data = DeepBookV3OrderBookData {
            timestamp: "123456".to_string(),
            bids: vec![input_db_bids],
            asks: vec![input_db_asks]
        };

        let orderbook_under_test = OrderBook::from(test_deep_book_data);

        assert_eq!(orderbook_under_test, OrderBook::new(
            vec![
                PriceLevel{
                    price: Rate(BigDecimal::from_str("2").unwrap()), 
                    quantity: Quantity(BigDecimal::from_str("3").unwrap())
                }
            ],
            vec![
                PriceLevel{
                    price: Rate(BigDecimal::from_str("1").unwrap()), 
                    quantity: Quantity(BigDecimal::from_str("2").unwrap())
                }
            ],
        ))
    }

}