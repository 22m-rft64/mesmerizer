use thiserror::Error;

#[derive(Error, Debug)]
pub enum MesError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(String),
    #[error("provider error: {0}")]
    Provider(String),
    #[error("not found: {0}")]
    NotFound(String),
}
