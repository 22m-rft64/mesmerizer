use mes_core::scaffold::scaffold;
use std::env;

pub fn run(category: String) -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    let path = scaffold(&category, &cwd).map_err(anyhow::Error::from)?;
    println!("created: {path}");
    Ok(())
}
