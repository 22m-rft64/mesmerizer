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
    /// Comprehensive system + env_refs + checks + mcp dump
    Doctor {
        #[arg(short, long)]
        category: Option<String>,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        no_color: bool,
    },
    /// Deploy a curated toolkit and/or patch MCP config
    Setup {
        /// env_ref name (from frontmatter `name:` field)
        name: String,
        #[arg(long)]
        apply: bool,
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
        TetoCmd::Doctor { category, json, no_color } => doctor_cmd(category, json, no_color),
        TetoCmd::Setup { name, apply } => setup_cmd(&name, apply),
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

fn doctor_cmd(
    category: Option<String>,
    json: bool,
    no_color: bool,
) -> anyhow::Result<()> {
    let report = mes_core::teto::doctor::collect(category.as_deref())
        .map_err(anyhow::Error::from)?;
    if json {
        let s = mes_core::teto::render::doctor_json(&report).map_err(anyhow::Error::from)?;
        println!("{s}");
    } else {
        header::print_sprite();
        println!();
        print!("{}", mes_core::teto::render::doctor_text(&report, no_color));
    }
    Ok(())
}

fn setup_cmd(name: &str, apply: bool) -> anyhow::Result<()> {
    let root = store::default_root();
    let refs = store::load_env_refs(&root).map_err(anyhow::Error::from)?;
    let er = refs
        .iter()
        .find(|r| r.name == name)
        .ok_or_else(|| anyhow::anyhow!("env_ref `{name}` not found in {}", root.display()))?;
    let plan = mes_core::teto::setup::plan_setup(er);
    print!("{}", mes_core::teto::setup::render_plan(&plan));
    if !apply {
        println!("\n(dry-run; pass --apply to execute)");
        return Ok(());
    }
    mes_core::teto::setup::apply(&plan, er).map_err(anyhow::Error::from)?;
    println!("\ndone.");
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
