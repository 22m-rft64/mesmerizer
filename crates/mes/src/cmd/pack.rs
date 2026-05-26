use mes_core::pack::pack;

pub fn run(spec: String) -> anyhow::Result<()> {
    let out = pack(&spec).map_err(anyhow::Error::from)?;
    print!("{out}");
    Ok(())
}
