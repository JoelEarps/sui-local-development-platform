mod errors;
mod market_data_collector;
mod config_handler;

use market_data_collector::request_handler::fetch_market_data_at_required_rate;
use config_handler::{ApplicationConfiguration, load_arguments};
use std::io::Error;
use sui_models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Fetch Application Configuration
    let application_arguments = load_arguments();
    let application_configuration = ApplicationConfiguration::new(application_arguments)?;

    // Periodically Fetch Data at the rate described the config
    let fetching_data_task = tokio::spawn(
        fetch_market_data_at_required_rate(application_configuration.base_url, application_configuration.api_call_rate)
    );

    // TODO: Perform Data Processing on incoming data.
    let test = sui_models::add(12, 13);
    println!("{test}");

    // Handle Multi threading
    let application_result = tokio::try_join!(fetching_data_task);
    match application_result {
        Ok(_) => Ok(()),
        // TODO: Make custom errors for failing to fetch, process and operate
        Err(_) => Err(anyhow::Error::new(Error::new(std::io::ErrorKind::Other, "Custom Errors to be made")))
    }
}
