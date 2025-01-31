use anyhow::Result;
use crate::{errors::data_fetching_errors::DataFetchingErrors, market_data_collector::deepbook_orderbook::DeepBookV3OrderBookData};
use connector_model::orderbook::OrderBook;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn testing_making_request(){
        let response = fetch_market_data_at_required_rate("https://deepbook-indexer.mainnet.mystenlabs.com/get_pools".to_string(), 1).await;
        println!("{:?}", response);
    }
}