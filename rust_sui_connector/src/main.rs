mod errors;
mod market_data_collector;
mod config_handler;
mod market_client_handler;

use std::{collections::HashMap, sync::Arc};
use config_handler::{ApplicationConfiguration, load_arguments};
use market_client_handler::SUIConnectors;
use market_data_collector::deepbook_connector::DeepBookConnector;




#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Fetch Application Configuration
    let application_arguments = load_arguments();
    let application_configuration = ApplicationConfiguration::new(application_arguments)?;

    // TODO: How are we handling other pairs
    // Pass in PubKey, what is pubkey used for here?
    // What functionality do we need, what should be tested?
    // Error scenarios?
    let initial_connector = Arc::new(DeepBookConnector::new(application_configuration.base_url, 2, 4, "SUI_USDC".to_string()));
    let mut market_connectors = SUIConnectors {
        sui_market_clients: HashMap::new()
    };

    market_connectors.sui_market_clients.insert("test_client", initial_connector);


    Ok(())
}
