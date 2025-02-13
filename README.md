# sheller

[![Crates.io](https://img.shields.io/crates/v/sheller.svg)](https://crates.io/crates/sheller)
[![Documentation](https://docs.rs/sheller/badge.svg)](https://docs.rs/sheller)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A flexible framework for building interactive terminal-based shell applications in Rust. `sheller` provides the foundation for creating custom shells with rich TUI features, command history, and customizable command execution.

## Features

- 🚀 Easy-to-use API for building custom shell applications
- 🎨 Built-in TUI with command history and interactive prompt
- ⌨️ Rich keyboard input handling with customizable keybindings
- 🔄 Asynchronous command execution support via Tokio
- 📝 Command completion support
- 📋 Command history with scrollback
- 🎯 Custom prompt styling and configuration
- ⚡ Zero-cost abstractions for command execution

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sheller = "0.1.0"
```

## Quick Start

Here's a simple example of creating an echo shell that repeats user input:

```rust
use sheller::{command, renderer};

struct EchoExecutor {}

impl command::New for EchoExecutor {
    fn new() -> anyhow::Result<(Self, Self::Context)> {
        Ok((Self {}, ()))
    }
}

impl command::Execute for EchoExecutor {
    type Context = ();

    fn prompt(&self, _ctx: &Self::Context) -> String {
        "$ ".to_string()
    }

    fn execute(
        &self,
        _ctx: &mut Self::Context,
        cmd: command::CommandInput,
    ) -> anyhow::Result<command::OutputAction> {
        Ok(command::OutputAction::Command(command::CommandOutput {
            prompt: cmd.prompt,
            command: cmd.command.clone(),
            stdin: cmd.stdin.unwrap_or_default(),
            stdout: vec![cmd.command],
            stderr: Vec::new(),
        }))
    }

    fn prepare(&self, cmd: &str) -> command::Prepare {
        command::Prepare {
            command: cmd.to_string(),
            stdin_required: false,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let app = renderer::App::<EchoExecutor>::new(rt)?;
    app.execute()
}
```

## Core Concepts

### Executor

The `Execute` trait is the core of `sheller`. It defines how your shell handles commands:

- `prompt`: Defines the shell prompt
- `prepare`: Pre-processes commands and determines if they need stdin
- `execute`: Handles actual command execution
- `completion`: Provides command completion suggestions (optional)

### Command Flow

1. User enters a command
2. `prepare` is called to determine command requirements
3. If stdin is required, user can input multiple lines
4. `execute` is called with the command and any stdin
5. Output is displayed in the TUI

### TUI Features

- Command history navigation
- Line editing with cursor movement
- Tab completion support
- Clear screen (Ctrl+L)
- Exit shell (Ctrl+D)
- Command interruption (Ctrl+C)

## Advanced Usage

### Custom Context

You can maintain state between commands using a custom context:

```rust
struct MyContext {
    variables: HashMap<String, String>,
}

struct MyExecutor {}

impl command::Execute for MyExecutor {
    type Context = MyContext;
    
    // Implementation details...
}
```

### Async Command Execution

`sheller` provides access to a Tokio runtime for async operations:

```rust
fn execute(
    &self,
    ctx: &mut Self::Context,
    cmd: command::CommandInput,
) -> anyhow::Result<command::OutputAction> {
    cmd.runtime.block_on(async {
        // Your async code here
    })
}
```

### Command Completion

Implement custom command completion:

```rust
fn completion(
    &self,
    ctx: &Self::Context,
    incomplete: &str,
) -> anyhow::Result<(String, Vec<String>)> {
    // Return (fixed_part, suggestions)
    Ok(("".to_string(), vec!["command1".to_string(), "command2".to_string()]))
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
