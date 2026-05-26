use mes_core::decode::{classify, decode_chain};
use std::io::Read;

pub fn run(text: Option<String>) -> anyhow::Result<()> {
    let input = match text {
        Some(s) => s,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    let trimmed = input.trim();
    let initial = classify(trimmed);
    println!("[classify] {trimmed}\n  -> {initial:?}");
    let result = decode_chain(trimmed);
    if result.steps.is_empty() {
        println!("[no decode]");
    } else {
        for (i, step) in result.steps.iter().enumerate() {
            println!("[step {}] {:?}: {}", i + 1, step.encoding, step.decoded);
            if let Some(note) = &step.note {
                println!("  note: {note}");
            }
        }
    }
    Ok(())
}
