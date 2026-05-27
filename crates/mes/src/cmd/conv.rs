use mes_core::convert::{apply, ConvOp};
use std::io::Read;

pub fn run(op: String, input: Option<String>) -> anyhow::Result<()> {
    let parsed = ConvOp::parse(&op).map_err(anyhow::Error::from)?;
    let text = match input {
        Some(s) => s,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    let out = apply(parsed, &text).map_err(anyhow::Error::from)?;
    println!("{out}");
    Ok(())
}
