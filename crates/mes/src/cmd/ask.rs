use std::io::Read;

use mes_core::provider::{dispatch, SendOpts};
use mes_core::template::{load_template, render::render};

pub fn run(template: Option<String>, provider: Option<String>) -> anyhow::Result<()> {
    let mut context = String::new();
    std::io::stdin().read_to_string(&mut context)?;
    let prompt = match template {
        Some(name) => {
            let tmpl = load_template(&name).map_err(anyhow::Error::from)?;
            render(&tmpl, &context)
        }
        None => context,
    };
    let provider_name = provider.unwrap_or_else(|| "claude_code".to_string());
    let response = dispatch(&provider_name, &prompt, &SendOpts::default())
        .map_err(anyhow::Error::from)?;
    print!("{response}");
    Ok(())
}
