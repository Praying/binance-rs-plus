use serde::Deserialize;
use thiserror::Error;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,
}

impl fmt::Display for BinanceContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, message: {}", self.code, self.msg)
    }
}

impl std::error::Error for BinanceContentError {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Binance API Error: {0}")]
    BinanceError(#[from] BinanceContentError),

    #[error("Invalid Kline Vec: {name} at index {index} is missing")]
    KlineValueMissingError { index: usize, name: &'static str },

    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Invalid Header Error: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse Float Error: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("URL Parse Error: {0}")]
    UrlParser(#[from] url::ParseError),

    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("WebSocket Error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("System Time Error: {0}")]
    Timestamp(#[from] std::time::SystemTimeError),

    #[error("Custom Error: {0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;
