use clap::Parser;
use config::Config;
use std::{collections::HashMap, path::Path};

use crate::errors::data_fetching_errors::DataFetchingErrors;

#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Args {
    #[arg(short, long, default_value_t = String::from("local") )]
    pub environment: String,

    #[arg(short, long, default_value_t = String::from("rust_sui_connector.json") )]
    pub file_name: String,

    #[arg(short, long, default_value_t = String::from("../environments") )]
    pub config_directory: String
}

pub(crate) fn load_arguments() -> Args {
    Args::parse()
}

type ApplicationConfigurationSerialised = HashMap<String, String>;

pub(crate) struct ApplicationConfiguration {  
    pub(crate) base_url: String,
    pub(crate)  api_call_rate: u64
}

const BASE_URL_KEY: &str = "base_url";
const FETCH_RATE_KEY: &str = "fetch_rate";

impl ApplicationConfiguration {
    pub(crate) fn new(application_arguments: Args) -> Result<Self, DataFetchingErrors> {
        let serialised_config = ApplicationConfiguration::fetch_configuration(application_arguments)?;

        Ok(ApplicationConfiguration{
            base_url: serialised_config.get(BASE_URL_KEY).expect("No base url present").to_string(),
            api_call_rate: serialised_config.get(FETCH_RATE_KEY).expect("No base url present").parse::<u64>()?
        })
    }

    /// Fetches the configuration for the application from the file path
    /// Examples
    fn fetch_configuration(application_arguments: Args) -> Result<ApplicationConfigurationSerialised, DataFetchingErrors> {
        println!("Fetching and validating Configuration");
        let config_path = Path::new(&application_arguments.config_directory).join(application_arguments.environment).join(application_arguments.file_name);
        println!("{:#?} ", config_path);

       let application_configuration = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().expect("Invalid Config Path Format")))
        .build()?;

        let serialised_config: ApplicationConfigurationSerialised = application_configuration.try_deserialize::<ApplicationConfigurationSerialised>()?;
        Ok(serialised_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Should Succeed using passed values - happy to run this as a unit test as it is inexpensive
    // Should fail with invalid passed values

    #[test]
    fn configuration_handling_should_succeed(){
        let test_arguments = Args {
            environment: "local".to_string(),
            file_name: "rust_sui_connector.json".to_string(),
            config_directory: "../environments".to_string(),
        };
        let test_config_handler = ApplicationConfiguration::new(test_arguments).unwrap();
        assert_eq!(test_config_handler.api_call_rate, 1);
        assert_eq!(test_config_handler.base_url, "https://deepbook-indexer.mainnet.mystenlabs.com/get_pools");
    }

    #[test]
    // #[should_panic]
    fn configuration_handling_should_fail(){
        let test_arguments = Args {
            environment: "fake_env".to_string(),
            file_name: "fake_config.json".to_string(),
            config_directory: "../environments".to_string(),
        };
        let test_config_handler = ApplicationConfiguration::new(test_arguments).unwrap();
        assert_eq!(test_config_handler.api_call_rate, 1);
        assert_eq!(test_config_handler.base_url, "https://deepbook-indexer.mainnet.mystenlabs.com/get_pools");
    }
}