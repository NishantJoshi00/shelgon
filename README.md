# Shelgon <img src="https://img.pokemondb.net/artwork/vector/shelgon.png" align="right" width="128" />

Shelgon is a robust Rust framework for building interactive REPL (Read-Eval-Print Loop) applications and custom shells. It provides a flexible, type-safe foundation with built-in terminal UI capabilities using `ratatui`.

[![Crates.io](https://img.shields.io/crates/v/shelgon.svg)](https://crates.io/crates/shelgon)
[![Documentation](https://docs.rs/shelgon/badge.svg)](https://docs.rs/shelgon)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🛡️ **Type-safe Command Execution** - Like Shelgon's protective shell, your commands are wrapped in a type-safe interface
- 🔄 **Async Runtime Integration** - Built on tokio for high-performance async operations
- 🎨 **Beautiful TUI** - Powered by ratatui with support for styling and colors
- ⌨️ **Rich Input Handling** - Complete keyboard interaction support including:
  - Command history
  - Cursor movement
  - Tab completion
  - Ctrl+C/Ctrl+D handling
- 📝 **Custom Context Support** - Maintain state between commands with your own context type
- 📥 **STDIN Support** - Handle multi-line input for commands that need it

## Installation

Add Shelgon to your `Cargo.toml`:

```toml
[dependencies]
shelgon = "0.1.0"
tokio = { version = "1.43.0", features = ["full"] }
anyhow = "1.0.95"
```

## Quick Start

Create a simple echo shell:

```rust
use shelgon::{command, renderer};

struct EchoExecutor {}

impl command::New for EchoExecutor {
    fn new() -> anyhow::Result<(Self, ())> {
        Ok((Self {}, ()))
    }
}

impl command::Execute for EchoExecutor {
    type Context = ();

    fn prompt(&self, _: &Self::Context) -> String {
        "$".to_string()
    }

    fn execute(
        &self,
        _: &mut Self::Context,
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
}

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let app = renderer::App::<EchoExecutor>::new(rt)?;
    app.execute()
}
```

## Evolution Guide: Building Your Own Shell <img src="https://img.pokemondb.net/artwork/vector/salamence.png" align="right" width="128" />

Here's how to build a dragon-like shell with `shelgon`:

1. **Define Your Executor**: Create a type that implements `command::Execute`
2. **Create Your Context**: Design a context type to maintain state between commands
3. **Implement Command Logic**: Add your command execution logic in the `execute` method
4. **Add Tab Completion**: Implement the `completion` method for smart suggestions
5. **Handle STDIN**: Use the `prepare` method to indicate which commands need input

## Examples

Check out the [examples](./examples) directory for more advanced usage patterns, including:

- `echosh.rs`: A basic echo shell demonstrating core functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Before contributing, please:

1. Check existing issues or create a new one
2. Fork the repository
3. Create your feature branch (`git checkout -b feature/amazing-feature`)
4. Commit your changes (`git commit -m 'Add some amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

Built by Human, Documented by LLM.

</div>
