use crate::teto::error::TetoError;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize)]
pub struct EnvRef {
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub activation: Vec<String>,
    #[serde(default)]
    pub check: Vec<CheckItem>,
    #[serde(default)]
    pub deploy: Option<Deploy>,
    #[serde(default)]
    pub repair: Vec<RepairItem>,
    #[serde(default)]
    pub invocations: Vec<Invocation>,
    #[serde(default)]
    pub mcp: Vec<McpServer>,
    #[serde(skip)]
    pub source_path: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckItem {
    pub id: String,
    pub desc: String,
    pub cmd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Deploy {
    pub source: DeploySource,
    pub install_dir: String,
    #[serde(default)]
    pub post_install: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DeploySource {
    Git {
        repo: String,
        #[serde(default = "default_branch")]
        branch: String,
        #[serde(default)]
        sparse_path: Option<String>,
    },
}

fn default_branch() -> String {
    "main".into()
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepairItem {
    pub id: String,
    pub desc: String,
    pub detect: String,
    pub fix: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Invocation {
    pub id: String,
    pub cmd: String,
    pub desc: String,
    #[serde(default)]
    pub args: Vec<InvocationArg>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InvocationArg {
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct McpServer {
    pub id: String,
    pub server_cmd: Vec<String>,
    #[serde(default = "default_transport")]
    pub transport: String,
    #[serde(default)]
    pub install_for: BTreeMap<String, McpInstall>,
}

fn default_transport() -> String {
    "stdio".into()
}

#[derive(Debug, Clone, Deserialize)]
pub struct McpInstall {
    pub config_path: String,
    pub block: serde_yaml::Value,
}

impl EnvRef {
    pub fn from_skill_md(text: &str) -> Result<Self, TetoError> {
        let (front, _body) = split_frontmatter(text).ok_or_else(|| TetoError::Parse {
            file: "<inline>".into(),
            msg: "no `---` frontmatter block found".into(),
        })?;
        let er: EnvRef = serde_yaml::from_str(front)?;
        Ok(er)
    }
}

fn split_frontmatter(text: &str) -> Option<(&str, &str)> {
    let stripped = text.strip_prefix("---\n")?;
    let end = stripped.find("\n---\n")?;
    Some((&stripped[..end], &stripped[end + 5..]))
}
