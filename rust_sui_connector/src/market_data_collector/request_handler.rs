use std::collections::HashMap;
use std::io::stderr;
use std::io::Error;
use std::io::ErrorKind;
use anyhow::anyhow;
use anyhow::Result;
use connector_model::orderbook::PriceLevel;
use connector_model::pricing::Quantity;
use connector_model::pricing::Rate;
use serde::Deserialize;
use crate::errors::data_fetching_errors::DataFetchingErrors;
use connector_model::orderbook::OrderBook;
use std::str::FromStr;
use bigdecimal::BigDecimal;

#[derive(Deserialize, Debug)]
pub struct DeepBookV3OrderBookData {
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

// Convert from DeepBook -> Generic OrderBook
// Make this a struct that implements the trait MarketBuilder
// Get head round difference between Market and current function

// OrderBook -> Vector of bids and ask of type PriceLevel, PriceLevel has two members, price and amount of type Rate and Quantity respectively.
// Therefore we need a way of converting to Price Levels to construct new OrderBook data.
pub(crate) async fn fetch_market_data_at_required_rate(base_url_from_config: String, rate: u64) -> Result<(), DataFetchingErrors> {
    println!("Fetching market data from DeepBook Indexer at a rate of {rate} seconds");
    let mut attempt_counter = 0;
    loop {
        println!("Attempt number {attempt_counter}");
        // Pop this on a mutex to see if it has been changed? Clever way of checking if this has changed?
        let test = make_request(base_url_from_config.clone(), "SUI_USDC", 4).await;
        match test {
            Ok(deepbook_order_book_data) => {
                let generic_orderbook = OrderBook::from(deepbook_order_book_data);
                println!("{:#?}", generic_orderbook);
            },
            Err(_) => attempt_counter+=1
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(rate)).await;
        attempt_counter+=1;
        if attempt_counter > 5 {
                return Err(DataFetchingErrors::RetryError(5))
        }
    }
}

pub(super) async fn make_request(base_url: String, market_id: &str, depth: u8) -> Result<DeepBookV3OrderBookData, DataFetchingErrors> {
    let url_with_params = format!("{}{}?level={}&depth={}",base_url, market_id, 2, depth);
    println!("Making request to {url_with_params}");
    let resp = reqwest::get(url_with_params)
        .await?
        .json::<DeepBookV3OrderBookData>()
        .await?;
    println!("{:?}", resp);
    Ok(resp)
}

/// Error Scenarios:
/// Successful Fetch
/// API error and retry limit exceeded
#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Provide mock implementation for request making, thus allowing for quicker and less expensive tests
    // Mockall has been used before
    // https://medium.com/@md.abir1203/rust-testing-frameworks-931ae101b3c7
    #[tokio::test]
    async fn testing_making_request(){
        let response = fetch_market_data_at_required_rate("https://deepbook-indexer.mainnet.mystenlabs.com/get_pools".to_string(), 1).await;
        println!("{:?}", response);
    }
}