pub mod chain;
pub mod classify;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Encoding {
    Hex,
    Base64,
    UrlEncoded,
    Decimal,
    Address,
    Ascii,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecodeResult {
    pub input: String,
    pub steps: Vec<DecodeStep>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecodeStep {
    pub encoding: Encoding,
    pub decoded: String,
    pub note: Option<String>,
}

pub use chain::decode_chain;
pub use classify::classify;
