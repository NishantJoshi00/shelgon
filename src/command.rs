//! Command execution and shell interaction primitives for building REPLs.
//!
//! This module provides the core traits and types needed to implement custom REPL (Read-Eval-Print Loop)
//! shells. The primary components are:
//!
//! - [`Execute`]: The main trait for implementing shell command execution
//! - [`CommandInput`]: Input data structure passed to command executors
//! - [`CommandOutput`]: Output data structure for command results
//! - [`OutputAction`]: Enum controlling shell behavior after command execution
//!
//! # Architecture
//!
//! The command execution flow follows these steps:
//!
//! 1. Shell displays a prompt (via [`Execute::prompt`])
//! 2. User enters a command
//! 3. Shell optionally handles tab completion (via [`Execute::completion`])
//! 4. Shell prepares command execution (via [`Execute::prepare`])
//! 5. Command is executed (via [`Execute::execute`])
//! 6. Output is rendered based on returned [`OutputAction`]
//!
//! # Example
//!
//! ```rust
//! use shelgon::command::{self, Execute, CommandInput, CommandOutput, OutputAction};
//!
//! struct MyExecutor {}
//!
//! impl Execute for MyExecutor {
//!     type Context = ();
//!
//!     fn prompt(&self, _: &Self::Context) -> String {
//!         "$ ".to_string()
//!     }
//!
//!     fn prepare(&self, cmd: &str) -> command::Prepare {
//!         command::Prepare {
//!             command: cmd.to_string(),
//!             stdin_required: false,
//!         }
//!     }
//!
//!     fn execute(
//!         &self,
//!         _: &mut Self::Context,
//!         input: CommandInput
//!     ) -> anyhow::Result<OutputAction> {
//!         Ok(OutputAction::Command(CommandOutput {
//!             prompt: input.prompt,
//!             command: input.command.clone(),
//!             stdin: Vec::new(),
//!             stdout: vec![format!("Executed: {}", input.command)],
//!             stderr: Vec::new(),
//!         }))
//!     }
//! }
//! ```
//!
//! # Features
//!
//! - **tokio**: Enables async runtime support via [`tokio::runtime::Runtime`] in [`CommandInput`]

#[cfg(feature = "tokio")]
use std::sync::Arc;
#[cfg(feature = "tokio")]
use tokio::runtime::Runtime;

///
/// [`CommandOutput`] is the output supplied to the renderer by the [`Execute`] trait.
///
/// `prompt` & `command` are the prompt and command that were executed.
/// `stdin` is the input that was supplied to the command. (optional)
/// `stdout` & `stderr` are the output of the command.
///
pub struct CommandOutput {
    /// The prompt that was displayed.
    pub prompt: String,
    /// The command that was executed
    pub command: String,
    /// The input that was supplied to the command. (optional)
    pub stdin: Vec<String>,
    /// The output of the command.
    pub stdout: Vec<String>,
    /// The error output of the command. (optional)
    pub stderr: Vec<String>,
}

///
/// [`OutputAction`] is the action that the renderer should take after receiving the output from
/// the [`Execute`] trait.
///
/// This is to perform specialized actions on the shell, altering the rendered output from within
/// the [`Execute`] trait.
///
///
pub enum OutputAction {
    /// Render the output of the command.
    Command(CommandOutput),
    /// Exit the shell.
    Exit,
    /// Clear the screen.
    Clear,
}

///
/// [`CommandInput`] is the input supplied to the [`Execute`] trait.
///
/// This is the input that is received by the renderer, and is passed to the [`Execute`] trait.
/// This happens when the user submits a command.
///
pub struct CommandInput {
    /// The prompt that was displayed. (This is actually provided by [`Execute`] trait)
    pub prompt: String,
    /// The command that is supplied by the user.
    pub command: String,
    /// The input that is supplied to the command. (optional)
    pub stdin: Option<Vec<String>>,
    #[cfg(feature = "tokio")]
    /// Supplying [`tokio::runtime::Runtime`] to the [`Execute`] trait. This is to facilitate
    /// executing [`std::future::Future`]s, creating [`tokio::task::JoinHandle`]s, etc.
    pub runtime: Arc<Runtime>,
}

///
/// [`Prepare`] is the output of the [`Execute::prepare`] method.
///
/// This prepares the renderer, in case if some special actions are required before executing the
/// command.
///
/// Supported behaviors:
/// - `stdin_required`: If the command requires stdin, the renderer should prompt the user for input.
///
#[derive(Debug, Clone)]
pub struct Prepare {
    /// The command that is to be executed.
    pub command: String,
    /// If the command requires stdin.
    pub stdin_required: bool,
}

///
/// [`Execute`] this is the heart of the shell. This is the trait that is implemented by the
/// commands that are to be executed.
///
/// This trait is responsible for:
/// - Prompting the user for input.
/// - Completing the command. (optional)
/// - Preparing the command for execution.
/// - Executing the command.
///
/// This is the only trait that is required by the user to implement to create a REPL.
///
///
pub trait Execute {
    /// This is the context that is maintained by the `App` struct. This is specific to your
    /// [`Execute`] trait. This can contain any data that is required by the command.
    ///
    /// The context is read only during the [`Execute::prompt`], [`Execute::completion`] & [`Execute::prepare`] method,
    /// and is mutable during the [`Execute::execute`] method.
    ///
    type Context;

    ///
    /// This is the prompt that is displayed to the user. This is the first thing that is
    /// displayed, and the user is expected to enter a command.
    ///
    fn prompt(&self, ctx: &Self::Context) -> String;

    ///
    /// This is the completion that is displayed to the user. This is displayed when the user
    /// presses the `Tab` key. This is optional, and can be left empty.
    ///
    /// The completion works in the following way:
    /// When the user presses the `Tab` key, the shell will call the [`Execute::completion`]
    /// method. This method returns 2 things (deterministic completion and non-deterministic
    /// completion).
    ///
    /// The deterministic completion is applied to the command, and the non-deterministic is shown
    /// below the command.
    ///
    fn completion(
        &self,
        _ctx: &Self::Context,
        _incomplete_command: &str,
    ) -> anyhow::Result<(String, Vec<String>)> {
        // completion + branches
        Ok((String::new(), Vec::new()))
    }

    ///
    /// This is the prepare method. This is called before executing the command. This is used to
    /// prepare the command for execution.
    ///
    /// This can be used to check if the command requires stdin, and prompt the user for input.
    ///
    fn prepare(&self, cmd: &str) -> Prepare;
    ///
    /// This is the execute method. This is called to execute the command. This is where the
    /// command is executed. This is where the command is executed, and the output is returned.
    ///
    fn execute(&self, ctx: &mut Self::Context, cmd: CommandInput) -> anyhow::Result<OutputAction>;
}

///
/// [`New`] is the trait that is implemented by the commands that are to be executed. This is used
/// to quickly create a new instance of the command.
///
pub trait New: Execute {
    /// This is the new method that is used to create a new instance of the command.
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized;
}
