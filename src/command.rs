use std::sync::Arc;

use tokio::runtime::Runtime;

pub struct CommandOutput {
    pub prompt: String,
    pub command: String,
    pub stdin: Vec<String>,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
}

pub struct CommandInput {
    pub prompt: String,
    pub command: String,
    pub stdin: Option<Vec<String>>,
    pub runtime: Option<Arc<Runtime>>,
}

pub struct Prepare {
    pub command: String,
    pub stdin_required: bool,
    pub is_async: bool,
}

pub trait Execute {
    type Context;
    fn prompt(&self, ctx: &Self::Context) -> String;
    fn prepare(&self, cmd: &str) -> Prepare;
    fn execute(&self, ctx: &mut Self::Context, cmd: CommandInput) -> anyhow::Result<CommandOutput>;
}

pub trait New: Execute {
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized;
}

pub mod echosh;
