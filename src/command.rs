use std::sync::Arc;

use tokio::runtime::Runtime;

pub struct CommandOutput {
    pub prompt: String,
    pub command: String,
    pub stdin: Vec<String>,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
}

pub enum OutputAction {
    Command(CommandOutput),
    Exit,
    Clear,
}

pub struct CommandInput {
    pub prompt: String,
    pub command: String,
    pub stdin: Option<Vec<String>>,
    pub runtime: Arc<Runtime>,
}

#[derive(Debug, Clone)]
pub struct Prepare {
    pub command: String,
    pub stdin_required: bool,
}

pub trait Execute {
    type Context;
    fn prompt(&self, ctx: &Self::Context) -> String;
    fn prepare(&self, cmd: &str) -> Prepare;
    fn execute(&self, ctx: &mut Self::Context, cmd: CommandInput) -> anyhow::Result<OutputAction>;
}

pub trait New: Execute {
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized;
}

pub mod echosh;
