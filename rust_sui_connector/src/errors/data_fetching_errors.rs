use std::{num::ParseIntError, string::ParseError};
use config::ConfigError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum DataFetchingErrors {
    #[error("Failed to fetch the data from the API")]
    FetchError(#[from] reqwest::Error),

    #[error("Retry limit exceeded ({0} retries)")]
    RetryError(u8),

    #[error("Could not retrieve the application configuration")]
    ConfigError(#[from] ConfigError),

    #[error("Could not parse the fetch rate")]
    FetchRateParseError(#[from] ParseIntError),
}