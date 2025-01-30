use std::num::ParseIntError;
use config::ConfigError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ConfigLoadingErrors {
    #[error("Could not retrieve the application configuration")]
    ConfigError(#[from] ConfigError),

    #[error("Could not parse the fetch rate")]
    FetchRateParseError(#[from] ParseIntError),
}