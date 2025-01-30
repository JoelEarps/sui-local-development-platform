use std::collections::HashMap;
use std::io::stderr;
use std::io::Error;
use std::io::ErrorKind;
use anyhow::anyhow;
use anyhow::Result;
use serde::Deserialize;
use crate::errors::data_fetching_errors::DataFetchingErrors;
use connector_model::orderbook::OrderBook;

// TODO: Look at Mocking once initial functionality is complete
// use mockall::predicate::*;
// use mockall::automock;
// use mockall_double::double;


// #[derive(Clone)]
// pub(crate) struct RequestHandler{
//     base_url: String
// }

// #[automock]
// pub trait MakeRequest{
//     async fn make_request(&self) -> Result<bool, ()>;
// }

// impl MakeRequest for RequestHandler {
//     async fn make_request(&self) -> Result<bool, ()> {
//         // Look at removing this, just getting it working for now
//         // get(self.base_url.clone());
//         println!("Making Request to DeepBook Indexer");
//         Ok(true) 
//     }
// }

// impl RequestHandler {
//     pub fn new(base_url_from_config: String) -> Self {
//         Self {
//             base_url: base_url_from_config
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct DeepBookOrderBookData {
    timestamp: String,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>
}

pub(crate) async fn fetch_market_data_at_required_rate(base_url_from_config: String, rate: u64) -> Result<(), DataFetchingErrors> {
    println!("Fetching market data from DeepBook Indexer at a rate of {rate} seconds");
    let mut attempt_counter = 0;
    loop {
        println!("Attempt number {attempt_counter}");
        // Pop this on a mutex to see if it has been changed? Clever way of checking if this has changed?
        let test = make_request(base_url_from_config.clone(), "SUI_USDC", 4).await;
        match test {
            Ok(_) => println!("{test:#?}"),
            Err(_) => attempt_counter+=1
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(rate)).await;
        attempt_counter+=1;
        if attempt_counter > 5 {
                return Err(DataFetchingErrors::RetryError(5))
        }
    }
}

pub(super) async fn make_request(base_url: String, market_id: &str, depth: u8) -> Result<DeepBookOrderBookData, DataFetchingErrors> {
    let url_with_params = format!("{}{}?level={}&depth={}",base_url, market_id, 2, depth);
    println!("Making request to {url_with_params}");
    let resp = reqwest::get(url_with_params)
        .await?
        .json::<DeepBookOrderBookData>()
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