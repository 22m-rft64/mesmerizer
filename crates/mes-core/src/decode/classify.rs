use super::Encoding;
use regex::Regex;

pub fn classify(s: &str) -> Encoding {
    let s = s.trim();
    if s.is_empty() {
        return Encoding::Unknown;
    }
    let lowered = s.strip_prefix("0x").unwrap_or(s);
    let hex_re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    let b64_re = Regex::new(r"^[A-Za-z0-9+/]+={0,2}$").unwrap();
    let url_re = Regex::new(r"%[0-9a-fA-F]{2}").unwrap();
    let dec_re = Regex::new(r"^-?\d+$").unwrap();
    if hex_re.is_match(lowered) && lowered.len() % 2 == 0 && lowered.len() >= 2 {
        // Address-like: 0x-prefixed and >= 8 hex digits.
        if s.starts_with("0x") && lowered.len() >= 8 {
            return Encoding::Address;
        }
        return Encoding::Hex;
    }
    if url_re.is_match(s) {
        return Encoding::UrlEncoded;
    }
    if b64_re.is_match(s) && s.len() >= 4 && s.len() % 4 == 0 {
        return Encoding::Base64;
    }
    if dec_re.is_match(s) {
        return Encoding::Decimal;
    }
    if s.is_ascii() {
        return Encoding::Ascii;
    }
    Encoding::Unknown
}
