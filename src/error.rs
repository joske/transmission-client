use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("transmission authentication needed")]
    TransmissionUnauthorized,
    #[error("transmission error")]
    TransmissionError(String),
    #[error("reqwest error")]
    NetworkError(#[from] reqwest::Error),
    #[error("serde_json error")]
    SerdeError(#[from] serde_json::Error),
}
