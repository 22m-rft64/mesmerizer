use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mes", version, about = "mesmerizer CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Pack a file:lines selection into a token-efficient markdown context
    Pack { spec: String },
    /// Ask a question to an AI provider with optional template
    Ask {
        #[arg(short, long)]
        template: Option<String>,
        #[arg(short, long)]
        provider: Option<String>,
    },
    /// Decode and classify text (hex/base64/url/addr)
    Decode { text: Option<String> },
    /// Scaffold a CTF solve template into current directory
    Scaffold { category: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Pack { spec } => println!("pack stub: {spec}"),
        Cmd::Ask { template, provider } => {
            println!("ask stub: template={template:?} provider={provider:?}")
        }
        Cmd::Decode { text } => println!("decode stub: {text:?}"),
        Cmd::Scaffold { category } => println!("scaffold stub: {category}"),
    }
    Ok(())
}
