use thiserror::Error;


#[derive(Error, Debug)]
pub enum DataFetchingErrors {
    #[error("Failed to fetch the data from the API")]
    FetchError(#[from] reqwest::Error),

    // #[error("Failed to fetch the data from the API")]
    // UrlParseError(#[from] reqwest::Error),
    
    #[error("Retry limit exceeded ({0} retries)")]
    RetryError(u8),
}