//! A basic echo shell example demonstrating core Shelgon functionality.
//!
//! This example implements a simple shell that:
//! - Echoes back any command entered
//! - Provides special handling for the "cat" command to demonstrate STDIN support
//! - Uses a minimal custom context
//!
//! # Usage
//!
//! ```bash
//! cargo run --example echosh
//! ```
//!
//! After running, you can:
//! - Type any command to see it echoed back
//! - Use the "cat" command to test multi-line input (Ctrl+D to finish)
//! - Press Ctrl+C or Ctrl+D to exit

/// An executor that echoes back commands
pub struct Executor {}

/// Empty context since this example doesn't need state
pub struct Context {}

impl shelgon::command::New for Executor {
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized,
    {
        // Initialize with empty context
        Ok((Self {}, Self::Context {}))
    }
}

impl shelgon::command::Execute for Executor {
    type Context = Context;

    fn prompt(&self, _ctx: &Self::Context) -> String {
        // Simple dollar sign prompt
        "$".to_string()
    }

    fn prepare(&self, cmd: &str) -> shelgon::command::Prepare {
        // Special handling for 'cat' command to demonstrate STDIN support
        if cmd == "cat" {
            return shelgon::command::Prepare {
                command: cmd.to_string(),
                stdin_required: true,
            };
        }

        // All other commands don't need STDIN
        shelgon::command::Prepare {
            command: cmd.to_string(),
            stdin_required: false,
        }
    }

    fn execute(
        &self,
        _ctx: &mut Self::Context,
        cmd: shelgon::command::CommandInput,
    ) -> anyhow::Result<shelgon::command::OutputAction> {
        // Echo the command back as output
        let output = shelgon::command::CommandOutput {
            prompt: cmd.prompt,
            command: cmd.command.clone(),
            stdin: cmd.stdin.unwrap_or_default(),
            stdout: vec![cmd.command],
            stderr: Vec::new(),
        };
        Ok(shelgon::command::OutputAction::Command(output))
    }
}

fn main() -> anyhow::Result<()> {
    // Initialize tokio runtime
    let rt = tokio::runtime::Runtime::new()?;
    // Create and run the shell
    let app = shelgon::renderer::App::<Executor>::new(rt)?;
    app.execute()
}
