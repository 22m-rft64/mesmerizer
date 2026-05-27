use clap::Subcommand;
use mes_core::teto::{check, header, mcp, render, store};

#[derive(Subcommand)]
pub enum TetoCmd {
    /// Check all env_refs against system state
    Check {
        #[arg(short, long)]
        category: Option<String>,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        no_color: bool,
    },
    /// MCP-related operations
    Mcp {
        #[command(subcommand)]
        cmd: McpCmd,
    },
}

#[derive(Subcommand)]
pub enum McpCmd {
    /// List MCP servers declared by env_refs
    List {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        no_color: bool,
    },
}

pub fn run(cmd: TetoCmd) -> anyhow::Result<()> {
    match cmd {
        TetoCmd::Check { category, json, no_color } => check_cmd(category, json, no_color),
        TetoCmd::Mcp { cmd } => match cmd {
            McpCmd::List { json, no_color } => mcp_list_cmd(json, no_color),
        },
    }
}

fn check_cmd(category: Option<String>, json: bool, no_color: bool) -> anyhow::Result<()> {
    let root = store::default_root();
    let mut refs = store::load_env_refs(&root).map_err(anyhow::Error::from)?;
    if let Some(filter) = &category {
        refs.retain(|r| store::category_matches(&r.category, filter));
    }
    let results = check::run_checks(&refs);
    if json {
        let s = render::check_json(&results).map_err(anyhow::Error::from)?;
        println!("{s}");
    } else {
        header::print_sprite();
        println!();
        print!("{}", render::check_text(&results, no_color));
    }
    Ok(())
}

fn mcp_list_cmd(json: bool, no_color: bool) -> anyhow::Result<()> {
    let root = store::default_root();
    let refs = store::load_env_refs(&root).map_err(anyhow::Error::from)?;
    let entries = mcp::collect_mcps(&refs);
    if json {
        let s = render::mcp_list_json(&entries).map_err(anyhow::Error::from)?;
        println!("{s}");
    } else {
        header::print_sprite();
        println!();
        print!("{}", render::mcp_list_text(&entries, no_color));
    }
    Ok(())
}
