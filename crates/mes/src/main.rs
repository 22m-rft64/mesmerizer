use clap::{Parser, Subcommand};

mod cmd;
use cmd::teto::TetoCmd;

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
    /// Decode and classify text (hex/base64/url/addr) with auto-chain
    Decode { text: Option<String> },
    /// Explicit CTF conversions (l2b, b2l, h2b, b2h, bin, b32d)
    Conv {
        /// Operation: l2b | b2l | h2b | b2h | bin | b32d
        op: String,
        /// Input text; if absent, read from stdin
        input: Option<String>,
    },
    /// Scaffold a CTF solve template into current directory
    Scaffold { category: String },
    /// Teto environment reference management
    Teto {
        #[command(subcommand)]
        cmd: TetoCmd,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Pack { spec } => cmd::pack::run(spec),
        Cmd::Ask { template, provider } => cmd::ask::run(template, provider),
        Cmd::Decode { text } => cmd::decode::run(text),
        Cmd::Conv { op, input } => cmd::conv::run(op, input),
        Cmd::Scaffold { category } => cmd::scaffold::run(category),
        Cmd::Teto { cmd } => cmd::teto::run(cmd),
    }
}
