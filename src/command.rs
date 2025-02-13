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
    pub prompt: String,
    pub command: String,
    pub stdin: Vec<String>,
    pub stdout: Vec<String>,
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
/// - `stdin_required`: If the command requires stdin, the renderer should prompt the user for
///     input.
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
    /// This is the context that is maintained by the [`App`] struct. This is specific to your
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

pub trait New: Execute {
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized;
}
