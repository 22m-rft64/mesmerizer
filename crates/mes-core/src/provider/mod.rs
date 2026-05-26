use crate::error::MesError;

pub mod claude_code;
pub mod codex;

#[derive(Debug, Clone, Default)]
pub struct SendOpts {
    pub timeout_secs: Option<u64>,
}

pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn send(&self, prompt: &str, opts: &SendOpts) -> Result<String, MesError>;
}

pub fn registry() -> Vec<Box<dyn Provider>> {
    vec![
        Box::new(claude_code::ClaudeCode::new()),
        Box::new(codex::Codex::new()),
    ]
}

pub fn dispatch(name: &str, prompt: &str, opts: &SendOpts) -> Result<String, MesError> {
    let providers = registry();
    let provider = providers
        .iter()
        .find(|p| p.name() == name)
        .ok_or_else(|| MesError::NotFound(format!("provider {name}")))?;
    provider.send(prompt, opts)
}
