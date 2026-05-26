use super::{classify, DecodeResult, DecodeStep, Encoding};
use base64::{engine::general_purpose, Engine};

const MAX_DEPTH: usize = 4;

pub fn decode_chain(input: &str) -> DecodeResult {
    let input_owned = input.to_string();
    let mut steps = Vec::new();
    let mut current = input.trim().to_string();
    for _ in 0..MAX_DEPTH {
        let enc = classify(&current);
        let decoded = match enc {
            Encoding::Hex => {
                let cleaned = current.strip_prefix("0x").unwrap_or(&current);
                match hex::decode(cleaned) {
                    Ok(bytes) => match String::from_utf8(bytes.clone()) {
                        Ok(s) if s.chars().all(|c| !c.is_control() || c.is_ascii_whitespace()) => s,
                        _ => format!("{bytes:02x?}"),
                    },
                    Err(_) => break,
                }
            }
            Encoding::Base64 => match general_purpose::STANDARD.decode(current.as_bytes()) {
                Ok(bytes) => match String::from_utf8(bytes.clone()) {
                    Ok(s) => s,
                    Err(_) => format!("{bytes:02x?}"),
                },
                Err(_) => break,
            },
            Encoding::UrlEncoded => urlencoding::decode(&current)
                .map(|c| c.into_owned())
                .unwrap_or_else(|_| current.clone()),
            Encoding::Address => {
                steps.push(DecodeStep {
                    encoding: enc,
                    decoded: current.clone(),
                    note: Some(address_note(&current)),
                });
                break;
            }
            Encoding::Decimal | Encoding::Ascii | Encoding::Unknown => break,
        };
        if decoded == current {
            break;
        }
        steps.push(DecodeStep {
            encoding: enc,
            decoded: decoded.clone(),
            note: None,
        });
        current = decoded;
    }
    DecodeResult {
        input: input_owned,
        steps,
    }
}

fn address_note(addr: &str) -> String {
    let hex = addr.strip_prefix("0x").unwrap_or(addr);
    let n = u64::from_str_radix(hex, 16).unwrap_or(0);
    if n == 0 {
        return "null pointer".into();
    }
    if (0x7f0000000000..=0x7fffffffffff).contains(&n) {
        return "stack/libc/heap region (high userspace)".into();
    }
    if (0x555555554000..=0x5555ffffffff).contains(&n) {
        return "PIE base (typical)".into();
    }
    if (0x400000..=0x500000).contains(&n) {
        return "non-PIE binary base (typical)".into();
    }
    "userspace address".into()
}
