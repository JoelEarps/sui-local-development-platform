use bigdecimal::ParseBigDecimalError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum DataFetchingErrors {
    #[error("Failed to fetch the data from the API")]
    FetchError(#[from] reqwest::Error),

    #[error("Failed to convert orderbook data to require format")]
    DeepBookV3DeserialisationError(),

    #[error("Failed to parse bid and quantity")]
    BigDecimalParsingError(#[from] ParseBigDecimalError),

    #[error("Retry limit exceeded ({0} retries)")]
    RetryError(u8),
}