use std::collections::HashMap;
use std::io::stderr;
use std::io::Error;
use std::io::ErrorKind;
use anyhow::anyhow;
use anyhow::Result;

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

// What are we trying to do
/*
1. Call a function that makes a request every second 
2. The function makes a get request to fetch a specific set of market data
3. The object that use the get requets can be 
 */

// TODO: Remove anyhow and make custom Error type, use anyhow at top level for proper error propagation 
pub(crate) async fn fetch_market_data_at_required_rate(base_url_from_config: String, rate: u64) -> Result<()> {
    println!("Fetching market data from DeepBook Indexer at a rate of {rate} seconds");
    let mut attempt_counter = 0;
    loop {
        println!("Attempt number {attempt_counter}");
        let test = make_request(base_url_from_config.clone()).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(rate)).await;
        attempt_counter+=1;
        if attempt_counter > 5 {
                return Err(anyhow!("Random test error"))
        }
    }
}

pub(super) async fn make_request(base_url: String) -> Result<()> {
    println!("Making request to {base_url}");
    let resp = reqwest::get(base_url)
        .await?
        .text()
        .await?;
    println!("{resp:#?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Provide mock implementation for request making, thus allowing for quicker and less expensive tests
    // Mockall has been used before
    // https://medium.com/@md.abir1203/rust-testing-frameworks-931ae101b3c7
    #[tokio::test]
    async fn testing_making_request(){
        let _ = fetch_market_data_at_required_rate("https://www.rust-lang.org".to_string(), 1).await;
    }
}