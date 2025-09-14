use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinanceError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error: {code} - {msg}")]
    Api { code: i32, msg: String },

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("WebSocket error: {0}")]
    WebSocket(String),
    
    #[error("Timeout error")]
    Timeout,
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, BinanceError>;

#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorResponse {
    pub code: i32,
    pub msg: String,
}

impl From<ApiErrorResponse> for BinanceError {
    fn from(err: ApiErrorResponse) -> Self {
        BinanceError::Api {
            code: err.code,
            msg: err.msg,
        }
    }
}
