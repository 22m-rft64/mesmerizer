use mes_core::decode::{classify, decode_chain, Encoding};

#[test]
fn classify_hex() {
    assert_eq!(classify("deadbeef"), Encoding::Hex);
    assert_eq!(classify("DEAD"), Encoding::Hex);
}

#[test]
fn classify_address() {
    assert_eq!(classify("0xdeadbeef"), Encoding::Address);
    assert_eq!(classify("0x7ffff7e1c000"), Encoding::Address);
}

#[test]
fn classify_base64() {
    assert_eq!(classify("ZGVhZGJlZWY="), Encoding::Base64);
}

#[test]
fn classify_url() {
    assert_eq!(classify("hello%20world"), Encoding::UrlEncoded);
}

#[test]
fn classify_decimal() {
    assert_eq!(classify("12345"), Encoding::Decimal);
    assert_eq!(classify("-42"), Encoding::Decimal);
}

#[test]
fn classify_unknown_ascii() {
    assert_eq!(classify("just some plain text"), Encoding::Ascii);
}

#[test]
fn chain_base64_to_hex() {
    let result = decode_chain("ZGVhZGJlZWY=");
    assert!(!result.steps.is_empty());
    assert_eq!(result.steps[0].decoded, "deadbeef");
}

#[test]
fn chain_url_to_text() {
    let result = decode_chain("hello%20world");
    assert!(!result.steps.is_empty());
    assert_eq!(result.steps[0].decoded, "hello world");
}

#[test]
fn chain_address_emits_note() {
    let result = decode_chain("0x7ffff7e1c000");
    assert_eq!(result.steps.len(), 1);
    assert!(result.steps[0].note.is_some());
}
