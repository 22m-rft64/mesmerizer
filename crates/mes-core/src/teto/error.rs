use thiserror::Error;

#[derive(Error, Debug)]
pub enum TetoError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("env_ref parse {file}: {msg}")]
    Parse { file: String, msg: String },
    #[error("env_ref `{0}` not found")]
    NotFound(String),
    #[error("shell command failed (exit {code:?}): {cmd}")]
    Shell { cmd: String, code: Option<i32> },
    #[error("config patch: {0}")]
    Config(String),
    #[error("yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}
