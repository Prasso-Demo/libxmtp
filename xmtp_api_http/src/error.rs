use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    pub code: usize,
    pub message: String,
    pub details: Vec<String>,
}

impl From<ErrorResponse> for HttpClientError {
    fn from(e: ErrorResponse) -> HttpClientError {
        HttpClientError::Grpc(e)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("grpc error {} at http gateway {}", _0.code, _0.message)]
    Grpc(ErrorResponse),
    #[error(transparent)]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    HeaderName(#[from] reqwest::header::InvalidHeaderName),
    #[error("error deserializing json response {0}")]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Decode(#[from] prost::DecodeError),
    #[error(transparent)]
    Uri(#[from] http::uri::InvalidUri),
    #[error(transparent)]
    InvalidUri(#[from] http::uri::InvalidUriParts),
    #[error(transparent)]
    Http(#[from] http::Error),
}

impl xmtp_common::RetryableError for HttpClientError {
    fn is_retryable(&self) -> bool {
        true
    }
}
