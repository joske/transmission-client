use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("transmission error")]
    TransmissionError(String),
    #[error("isahc network error")]
    NetworkError(#[from] isahc::Error),
    #[error("isahc http error")]
    HttpError(#[from] isahc::http::Error),
    #[error("serde_json error")]
    SerdeError(#[from] serde_json::Error),
}
