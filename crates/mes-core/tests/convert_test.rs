use mes_core::convert::{apply, ConvOp};

#[test]
fn parse_op_aliases() {
    assert_eq!(ConvOp::parse("l2b").unwrap(), ConvOp::LongToBytes);
    assert_eq!(ConvOp::parse("long_to_bytes").unwrap(), ConvOp::LongToBytes);
    assert_eq!(ConvOp::parse("b2l").unwrap(), ConvOp::BytesToLong);
    assert_eq!(ConvOp::parse("h2b").unwrap(), ConvOp::HexToBytes);
    assert_eq!(ConvOp::parse("b2h").unwrap(), ConvOp::BytesToHex);
    assert_eq!(ConvOp::parse("bin").unwrap(), ConvOp::BinaryToBytes);
    assert_eq!(ConvOp::parse("b32d").unwrap(), ConvOp::Base32Decode);
    assert!(ConvOp::parse("nope").is_err());
}

#[test]
fn long_to_bytes_decimal() {
    let out = apply(ConvOp::LongToBytes, "3735928559").unwrap();
    assert!(out.contains("deadbeef"));
    assert!(out.contains("4 bytes"));
}

#[test]
fn long_to_bytes_hex() {
    let out = apply(ConvOp::LongToBytes, "0xdeadbeef").unwrap();
    assert!(out.contains("deadbeef"));
}

#[test]
fn long_to_bytes_zero() {
    let out = apply(ConvOp::LongToBytes, "0").unwrap();
    assert!(out.contains("hex: 00"));
}

#[test]
fn bytes_to_long_from_hex() {
    let out = apply(ConvOp::BytesToLong, "deadbeef").unwrap();
    assert!(out.contains("decimal: 3735928559"));
    assert!(out.contains("0xdeadbeef"));
}

#[test]
fn bytes_to_long_from_hex_prefixed() {
    let out = apply(ConvOp::BytesToLong, "0xdeadbeef").unwrap();
    assert!(out.contains("decimal: 3735928559"));
}

#[test]
fn hex_to_bytes_ascii() {
    let out = apply(ConvOp::HexToBytes, "48656c6c6f").unwrap();
    assert!(out.contains("Hello"));
    assert!(out.contains("5 bytes"));
}

#[test]
fn hex_to_bytes_with_spaces() {
    let out = apply(ConvOp::HexToBytes, "48 65 6c 6c 6f").unwrap();
    assert!(out.contains("Hello"));
}

#[test]
fn bytes_to_hex_ascii() {
    let out = apply(ConvOp::BytesToHex, "Hello").unwrap();
    assert!(out.contains("hex: 48656c6c6f"));
    assert!(out.contains("5 bytes"));
}

#[test]
fn binary_to_bytes_one_byte() {
    let out = apply(ConvOp::BinaryToBytes, "01000001").unwrap();
    assert!(out.contains("repr: \"A\""));
}

#[test]
fn binary_to_bytes_multiple_with_spaces() {
    let out = apply(ConvOp::BinaryToBytes, "01000001 01000010 01000011").unwrap();
    assert!(out.contains("repr: \"ABC\""));
}

#[test]
fn binary_invalid_length_errors() {
    assert!(apply(ConvOp::BinaryToBytes, "0100001").is_err());
}

#[test]
fn base32_decode_simple() {
    // "Hello" base32 = "JBSWY3DPEE======"
    let out = apply(ConvOp::Base32Decode, "JBSWY3DPEE======").unwrap();
    assert!(out.contains("Hello"));
}

#[test]
fn long_to_bytes_handles_rsa_2048_sized_int() {
    // 2048-bit number — well beyond u128 (16 bytes).
    let big = "0x".to_string()
        + &"de".repeat(256); // 256 bytes = 2048 bits
    let out = apply(ConvOp::LongToBytes, &big).unwrap();
    assert!(out.contains("256 bytes"));
    assert!(out.contains(&"de".repeat(256)));
}

#[test]
fn long_to_bytes_with_underscore_separator() {
    let out = apply(ConvOp::LongToBytes, "3_735_928_559").unwrap();
    assert!(out.contains("deadbeef"));
}

#[test]
fn bytes_to_long_round_trip_large() {
    let huge_hex = "ff".repeat(128); // 1024-bit value (all 0xff)
    let b2l_out = apply(ConvOp::BytesToLong, &huge_hex).unwrap();
    // 2^1024 - 1 in decimal starts with "17976931..."
    assert!(b2l_out.contains("decimal: 1797"));
    assert!(b2l_out.contains(&format!("0x{}", huge_hex)));
    assert!(b2l_out.contains("128 bytes"));
}
