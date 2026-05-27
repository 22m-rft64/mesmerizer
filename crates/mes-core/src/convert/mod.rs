// Explicit CTF crypto conversions (CLI: `mes conv <op> <input>`).
use crate::error::MesError;
use base64::{engine::general_purpose, Engine};
use num_bigint::BigUint;
use num_traits::Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvOp {
    LongToBytes,
    BytesToLong,
    HexToBytes,
    BytesToHex,
    BinaryToBytes,
    Base32Decode,
}

impl ConvOp {
    pub fn parse(s: &str) -> Result<Self, MesError> {
        match s {
            "l2b" | "long2bytes" | "long_to_bytes" => Ok(ConvOp::LongToBytes),
            "b2l" | "bytes2long" | "bytes_to_long" => Ok(ConvOp::BytesToLong),
            "h2b" | "hex2bytes" => Ok(ConvOp::HexToBytes),
            "b2h" | "bytes2hex" => Ok(ConvOp::BytesToHex),
            "bin" | "binary" | "bin2bytes" => Ok(ConvOp::BinaryToBytes),
            "b32d" | "base32" | "base32decode" => Ok(ConvOp::Base32Decode),
            other => Err(MesError::NotFound(format!("conv op: {other}"))),
        }
    }
}

pub fn apply(op: ConvOp, input: &str) -> Result<String, MesError> {
    match op {
        ConvOp::LongToBytes => long_to_bytes(input),
        ConvOp::BytesToLong => bytes_to_long(input),
        ConvOp::HexToBytes => hex_to_bytes(input),
        ConvOp::BytesToHex => bytes_to_hex(input),
        ConvOp::BinaryToBytes => binary_to_bytes(input),
        ConvOp::Base32Decode => base32_decode(input),
    }
}

fn long_to_bytes(input: &str) -> Result<String, MesError> {
    let s = input.trim();
    // Strip underscores commonly used as digit separators in long literals.
    let cleaned = s.replace('_', "");
    let n = if let Some(hex) = cleaned.strip_prefix("0x") {
        BigUint::from_str_radix(hex, 16)
            .map_err(|e| MesError::Parse(format!("hex int parse: {e}")))?
    } else {
        BigUint::from_str_radix(&cleaned, 10)
            .map_err(|e| MesError::Parse(format!("decimal int parse: {e}")))?
    };
    let bytes = if n == BigUint::from(0u8) {
        vec![0u8]
    } else {
        n.to_bytes_be()
    };
    let hex = hex::encode(&bytes);
    let repr = printable_repr(&bytes);
    Ok(format!(
        "hex: {hex}\nrepr: {repr}\nlen: {} bytes",
        bytes.len()
    ))
}

fn bytes_to_long(input: &str) -> Result<String, MesError> {
    let bytes = parse_bytes_input(input)?;
    let n = BigUint::from_bytes_be(&bytes);
    Ok(format!(
        "decimal: {}\nhex: 0x{}\nlen: {} bytes",
        n.to_str_radix(10),
        n.to_str_radix(16),
        bytes.len()
    ))
}

fn hex_to_bytes(input: &str) -> Result<String, MesError> {
    let cleaned = input.trim().replace([' ', '_', '\n'], "");
    let cleaned = cleaned.strip_prefix("0x").unwrap_or(&cleaned).to_string();
    let bytes = hex::decode(&cleaned).map_err(|e| MesError::Parse(format!("hex: {e}")))?;
    let repr = printable_repr(&bytes);
    Ok(format!("bytes: {repr}\nlen: {} bytes", bytes.len()))
}

fn bytes_to_hex(input: &str) -> Result<String, MesError> {
    let bytes = input.as_bytes();
    Ok(format!("hex: {}\nlen: {} bytes", hex::encode(bytes), bytes.len()))
}

fn binary_to_bytes(input: &str) -> Result<String, MesError> {
    let cleaned: String = input.chars().filter(|c| *c == '0' || *c == '1').collect();
    if cleaned.is_empty() {
        return Err(MesError::Parse("no binary digits found".into()));
    }
    if cleaned.len() % 8 != 0 {
        return Err(MesError::Parse(format!(
            "binary length {} not a multiple of 8",
            cleaned.len()
        )));
    }
    let mut bytes = Vec::with_capacity(cleaned.len() / 8);
    for chunk in cleaned.as_bytes().chunks(8) {
        let s = std::str::from_utf8(chunk).unwrap();
        let b = u8::from_str_radix(s, 2)
            .map_err(|e| MesError::Parse(format!("binary chunk {s}: {e}")))?;
        bytes.push(b);
    }
    let repr = printable_repr(&bytes);
    Ok(format!("hex: {}\nrepr: {repr}\nlen: {} bytes", hex::encode(&bytes), bytes.len()))
}

fn base32_decode(input: &str) -> Result<String, MesError> {
    // base64 crate does not handle base32; do manual decode.
    let cleaned: String = input
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_ascii_uppercase();
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let stripped = cleaned.trim_end_matches('=');
    let mut bits = String::new();
    for c in stripped.chars() {
        let pos = alphabet
            .iter()
            .position(|&a| a == c as u8)
            .ok_or_else(|| MesError::Parse(format!("invalid base32 char: {c}")))?;
        bits.push_str(&format!("{:05b}", pos));
    }
    let mut bytes = Vec::with_capacity(bits.len() / 8);
    for chunk in bits.as_bytes().chunks(8) {
        if chunk.len() < 8 {
            break;
        }
        let s = std::str::from_utf8(chunk).unwrap();
        let b = u8::from_str_radix(s, 2)
            .map_err(|e| MesError::Parse(format!("base32 byte: {e}")))?;
        bytes.push(b);
    }
    let repr = printable_repr(&bytes);
    Ok(format!("bytes: {repr}\nhex: {}\nlen: {} bytes", hex::encode(&bytes), bytes.len()))
}

/// Accept bytes input as hex string (with optional 0x prefix) or as escaped Python-style
/// (e.g. `\xde\xad`) — falls back to interpreting as a raw ASCII string.
fn parse_bytes_input(input: &str) -> Result<Vec<u8>, MesError> {
    let s = input.trim();
    // 0xhex or pure hex string
    let candidate = s.strip_prefix("0x").unwrap_or(s);
    let candidate_no_ws = candidate.replace([' ', '_', '\n'], "");
    if !candidate_no_ws.is_empty()
        && candidate_no_ws.len() % 2 == 0
        && candidate_no_ws.chars().all(|c| c.is_ascii_hexdigit())
    {
        if let Ok(b) = hex::decode(&candidate_no_ws) {
            return Ok(b);
        }
    }
    // base64 attempt
    if let Ok(b) = general_purpose::STANDARD.decode(s.as_bytes()) {
        if !b.is_empty() {
            return Ok(b);
        }
    }
    // Fall through: treat as raw ASCII bytes
    Ok(s.as_bytes().to_vec())
}

fn printable_repr(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() + 4);
    out.push('"');
    for b in bytes {
        match *b {
            0x09 => out.push_str("\\t"),
            0x0a => out.push_str("\\n"),
            0x0d => out.push_str("\\r"),
            0x22 => out.push_str("\\\""),
            0x5c => out.push_str("\\\\"),
            0x20..=0x7e => out.push(*b as char),
            _ => out.push_str(&format!("\\x{:02x}", b)),
        }
    }
    out.push('"');
    out
}
