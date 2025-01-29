use std::io::Error;

use market_data_collector::request_handler::fetch_market_data_at_required_rate;
use sui_models;
use tokio::task::JoinSet;
mod market_data_collector;
use anyhow::Result;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: Fetch this from either env for config to enable testing
    let base_url_from_config = String::from("https://deepbook-indexer.mainnet.mystenlabs.com/");
    let fetching_rate = 1;
    let fetching_data_task = tokio::spawn(
        fetch_market_data_at_required_rate(base_url_from_config, fetching_rate)
    );
    let test = sui_models::add(12, 13);
    println!("{test}");
    let application_result = tokio::try_join!(fetching_data_task);
    match application_result {
        Ok(_) => Ok(()),
        // TODO: Make custom errors for failing to fetch, process and operate
        Err(_) => Err(anyhow::Error::new(Error::new(std::io::ErrorKind::Other, "Custom Errors to be made")))
    }
}
