//! # Shelgon
//!
//! A robust framework for building interactive REPL (Read-Eval-Print Loop) applications and custom shells in Rust.
//! Just as Shelgon evolves into the mighty Salamence, your REPL can evolve into a powerful shell.
//!
//! ## Overview
//!
//! Shelgon provides a flexible foundation for building terminal-based interactive applications with:
//!
//! - Type-safe command execution
//! - Beautiful terminal UI powered by `ratatui`
//! - Async runtime support via `tokio`
//! - Rich input handling and command history
//! - Tab completion support
//! - Custom context and state management
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use shelgon::{command, renderer};
//! use tokio::runtime::Runtime;
//!
//! // Define your command executor
//! struct EchoExecutor {}
//!
//! // Implement the core Execute trait
//! impl command::Execute for EchoExecutor {
//!     type Context = ();
//!
//!     fn prompt(&self, _: &Self::Context) -> String {
//!         "$".to_string()
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
//!         input: command::CommandInput,
//!     ) -> anyhow::Result<command::OutputAction> {
//!         Ok(command::OutputAction::Command(command::CommandOutput {
//!             prompt: input.prompt,
//!             command: input.command.clone(),
//!             stdin: Vec::new(),
//!             stdout: vec![input.command],
//!             stderr: Vec::new(),
//!         }))
//!     }
//! }
//!
//! // Optionally implement the New trait for convenient initialization
//! impl command::New for EchoExecutor {
//!     fn new() -> anyhow::Result<(Self, Self::Context)> {
//!         Ok((Self {}, ()))
//!     }
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     let rt = Runtime::new()?;
//!     let app = renderer::App::<EchoExecutor>::new(rt)?;
//!     app.execute()
//! }
//! ```
//!
//! ## Key Features
//!
//! ### Type-safe Command Execution
//!
//! Shelgon enforces type safety through its trait system:
//!
//! ```rust
//! # use shelgon::command::{self, Execute, CommandInput, OutputAction};
//! # struct MyExecutor {}
//! impl Execute for MyExecutor {
//!     type Context = Vec<String>; // Your custom context type
//!
//!     fn execute(
//!         &self,
//!         ctx: &mut Self::Context,
//!         input: CommandInput,
//!     ) -> anyhow::Result<OutputAction> {
//!         // Your command logic here
//!         # Ok(OutputAction::Exit)
//!     }
//!     // ... other required methods
//!     # fn prompt(&self, _: &Self::Context) -> String { "$ ".to_string() }
//!     # fn prepare(&self, cmd: &str) -> command::Prepare {
//!     #     command::Prepare { command: cmd.to_string(), stdin_required: false }
//!     # }
//! }
//! ```
//!
//! ### Rich Terminal UI
//!
//! Built on `ratatui`, Shelgon provides:
//!
//! - Command history with color-coded output
//! - Interactive command editing
//! - Tab completion interface
//! - Multi-line input support
//! - Error highlighting
//!
//! ### Async Support
//!
//! Seamless integration with `tokio` for async operations:
//!
//! ```rust,ignore
//! # use shelgon::command::{self, Execute, CommandInput, OutputAction};
//! # use tokio::runtime::Runtime;
//! # struct AsyncExecutor {}
//! # impl Execute for AsyncExecutor {
//! #     type Context = ();
//! #     fn prompt(&self, _: &Self::Context) -> String { "$ ".to_string() }
//! #     fn prepare(&self, cmd: &str) -> command::Prepare {
//! #         command::Prepare { command: cmd.to_string(), stdin_required: false }
//! #     }
//! async fn fetch_data() -> anyhow::Result<String> {
//!     // Async operations
//!     # Ok("data".to_string())
//! }
//!
//! impl Execute for AsyncExecutor {
//!     // ...
//!     fn execute(&self, _: &mut Self::Context, input: CommandInput) -> anyhow::Result<OutputAction> {
//!         let result = input.runtime.block_on(fetch_data())?;
//!         // Process result
//!         # Ok(OutputAction::Exit)
//!     }
//! }
//! # }
//! ```
//!
//! ## Core Modules
//!
//! - [`command`]: Core traits and types for command execution
//! - [`renderer`]: Terminal UI and application state management
//!
//! ## Features
//!
//! - `tokio`: Enables async runtime support (enabled by default)
//!
//! ## Shell Capabilities
//!
//! Shelgon shells support:
//!
//! - Command history
//! - Tab completion
//! - Multi-line input
//! - STDIN handling
//! - Color-coded output
//! - Error handling
//! - Screen clearing
//! - Custom prompt formatting
//!
//! ## Key Bindings
//!
//! - `Ctrl+L`: Clear screen
//! - `Ctrl+C/Ctrl+D`: Exit shell
//! - `Left/Right`: Move cursor
//! - `Tab`: Command completion
//! - `Enter`: Execute command or add STDIN line
//! - `Backspace`: Delete character
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

pub mod command;
pub mod renderer;

pub use command::*;
pub use renderer::App;
