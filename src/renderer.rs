//! Terminal UI renderer and application state management for REPL shells.
//!
//! This module provides the core [`App`] type which manages the terminal user interface,
//! handles user input, and coordinates command execution. It uses the `ratatui` library
//! for terminal rendering and `crossterm` for terminal manipulation.
//!
//! # Features
//!
//! - Terminal UI with command history
//! - Command input with cursor movement
//! - Tab completion support
//! - Multi-line input for commands requiring STDIN
//! - Color-coded output (command prompt, errors)
//! - Screen clearing and alternate screen support
//!
//! # Terminal UI States
//!
//! The renderer operates in two primary states:
//!
//! - **Idle**: Accepting user input with command editing capabilities
//! - **Running**: Processing command execution with optional STDIN input
//!
//! # Key Bindings
//!
//! The following key combinations are supported:
//!
//! - `Ctrl+L`: Clear screen
//! - `Ctrl+C/Ctrl+D`: Exit shell (or terminate current command if running)
//! - `Left/Right`: Move cursor
//! - `Tab`: Trigger command completion
//! - `Enter`: Execute command or add new STDIN line
//! - `Backspace`: Delete character
//!
//! # Example
//!
//! ```rust,ignore
//! use shelgon::renderer::App;
//! use shelgon::command::{self, Execute, CommandOutput, OutputAction};
//! use tokio::runtime::Runtime;
//!
//! struct MyExecutor {}
//!
//! impl command::Execute for MyExecutor {
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
//!     ) -> anyhow::Result<OutputAction> {
//!         Ok(OutputAction::Command(CommandOutput {
//!             prompt: input.prompt,
//!             command: input.command,
//!             stdin: Vec::new(),
//!             stdout: vec!["Hello, world!".to_string()],
//!             stderr: Vec::new(),
//!         }))
//!     }
//! }
//!
//! fn main() -> anyhow::Result<()> {
//!     let rt = Runtime::new()?;
//!     let app = App::new_with_executor(rt, MyExecutor {}, ());
//!     app.execute()
//! }
//! ```
//!
//! # Features
//!
//! - **tokio**: Enables async runtime support via [`tokio::runtime::Runtime`]
//!
//!

use std::io;
#[cfg(feature = "tokio")]
use std::sync::Arc;

use crossterm::{
    event::{KeyCode, KeyModifiers, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::CrosstermBackend,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame, Terminal,
};
#[cfg(feature = "tokio")]
use tokio::runtime::Runtime;

use crate::command::{self};

///
/// [`App`] is the main application.
///
/// commands. This is the main struct that is used to create a shell. This is responsible for
/// managing state, rendering the shell, and executing the commands.
///
pub struct App<T: command::Execute> {
    /// The executor that is used to execute the commands.
    executor: T,
    /// The context that is maintained by the [`App`] struct. This is specific to your
    context: T::Context,
    /// The state of the shell. This is different from the context. This is used to maintain
    /// information about the renderer.
    state: State,
    #[cfg(feature = "tokio")]
    /// The runtime that is passed to the `Execute` trait. This is used to facilitate executing
    /// on [`std::future::Future`]s, creating [`tokio::task::JoinHandle`]s, etc.
    runtime: Arc<Runtime>,
    /// The history of the commands that are executed.
    history: Vec<command::CommandOutput>,
}

/// The state of the shell.
enum State {
    /// The shell is idle. This is the default state of the shell.
    /// This is when the user is typing the command. This state holds the incomplete command, the
    /// cursor location, and the completions.
    Idle(String, usize, Option<Vec<String>>),
    /// The shell is running. This is when the command is being executed. This state holds the
    /// stdin that is being supplied to the command. And the contextual information about the
    /// command.
    Running(command::Prepare, Vec<String>),
}

///
/// The next action that is to be taken by the shell. As this is a REPL, this action decides
/// whether to continue the execution or to exit the shell.
#[derive(Debug, Default)]
enum Next {
    /// Continue the execution of the shell.
    #[default]
    Continue,
    /// Exit the shell.
    Exit(String),
    /// Clear renderer buffer
    Clear,
}

impl<T: command::Execute> App<T> {
    /// Create a new instance of the [`App`] struct.
    pub fn new(#[cfg(feature = "tokio")] rt: Runtime) -> anyhow::Result<Self>
    where
        T: command::New,
    {
        let (executor, context) = T::new()?;
        Ok(Self::new_with_executor(
            #[cfg(feature = "tokio")]
            rt,
            executor,
            context,
        ))
    }

    /// Create a new instance of the [`App`] struct with the executor and the context.
    pub fn new_with_executor(
        #[cfg(feature = "tokio")] rt: Runtime,
        executor: T,
        context: T::Context,
    ) -> Self {
        Self {
            executor,
            context,
            state: State::Idle(String::new(), 0, None),
            #[cfg(feature = "tokio")]
            runtime: Arc::new(rt),
            history: Vec::new(),
        }
    }

    /// Render the shell.
    fn render(&self, frame: &mut Frame) {
        let prompt = self.executor.prompt(&self.context);
        let area = frame.area();
        let mut text_content = self
            .history
            .iter()
            .flat_map(render_history)
            .collect::<Vec<_>>();

        match &self.state {
            State::Idle(ref cmd, cursor, comp) => {
                let (left_cmd, right_cmd) = cmd.split_at(*cursor);
                let left_cmd = Span::styled(left_cmd, Style::default().bold());
                let (cursor, right_cmd) = match right_cmd {
                    "" => {
                        let cursor =
                            Span::styled(" ", Style::default().bg(ratatui::style::Color::White));
                        let right_cmd = Span::raw("");
                        (cursor, right_cmd)
                    }
                    right_cmd => {
                        let cursor = Span::styled(
                            //
                            // # Safety: `right_cmd` will never be empty.
                            //
                            #[allow(clippy::expect_used)]
                            right_cmd
                                .chars()
                                .next()
                                .expect("match statement failed")
                                .to_string(),
                            Style::default()
                                .bg(ratatui::style::Color::White)
                                .fg(ratatui::style::Color::Black),
                        );

                        let right_cmd =
                            Span::styled(right_cmd[1..].to_string(), Style::default().bold());
                        (cursor, right_cmd)
                    }
                };

                text_content.push(Line::from(vec![
                    Span::styled(prompt.clone(), Style::default().blue()),
                    Span::raw(" "),
                    Span::styled(left_cmd.to_string(), Style::default().bold()),
                    cursor,
                    right_cmd,
                ]));

                if let Some(comp) = comp {
                    let completions = comp
                        .iter()
                        .map(|cmp| cmd.to_string() + cmp)
                        .map(|line| {
                            Span::styled(
                                line,
                                Style::default().bg(ratatui::style::Color::Rgb(200, 200, 200)),
                            )
                        })
                        .map(Line::from)
                        .collect::<Vec<_>>();
                    text_content.extend(completions);
                }

                let text_para = Paragraph::new(text_content).wrap(Wrap { trim: true });
                frame.render_widget(text_para, area);
            }
            State::Running(ref prep, stdin) => {
                text_content.push(Line::from(vec![
                    Span::styled(prompt.clone(), Style::default().blue()),
                    Span::raw(" "),
                    Span::styled(prep.command.clone(), Style::default().bold()),
                ]));
                let stdin = stdin
                    .iter()
                    .map(Span::raw)
                    .map(Line::from)
                    .collect::<Vec<_>>();
                text_content.extend(stdin);

                let history_para = Paragraph::new(text_content).wrap(Wrap { trim: true });
                frame.render_widget(history_para, area);
            }
        }
    }

    /// Handle the input from the user.
    fn input(&mut self, event: crossterm::event::Event) -> anyhow::Result<Next> {
        if let crossterm::event::Event::Key(ke) = event {
            match (ke.code, ke.modifiers, ke.kind) {
                (_, _, KeyEventKind::Release) => {
                    // Ignore Release events, prevents getting double keypresses on windows
                }
                (KeyCode::Char('l'), KeyModifiers::CONTROL, _) => {
                    self.history.clear();
                    return Ok(Next::Continue);
                }

                (KeyCode::Char('d') | KeyCode::Char('c'), KeyModifiers::CONTROL, _) => {
                    if let State::Running(..) = &self.state {
                        self.continue_execution()?;
                    } else {
                        return Ok(Next::Exit("".to_string()));
                    }
                }
                (KeyCode::Left, KeyModifiers::NONE, _) => self.move_cursor_left(),
                (KeyCode::Right, KeyModifiers::NONE, _) => self.move_cursor_right(),
                (KeyCode::Tab, KeyModifiers::NONE, _) => {
                    if let State::Idle(ref mut cmd, ref mut cursor, ref mut comp @ None) =
                        self.state
                    {
                        if *cursor == cmd.len() {
                            let (fixed, variable) = self.executor.completion(&self.context, cmd)?;
                            cmd.push_str(&fixed);
                            *cursor = cmd.len();
                            *comp = Some(variable);
                        }
                    }
                }
                (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT, _) => match self.state {
                    State::Idle(ref mut cmd, ref mut cursor, ref mut comp) => {
                        cmd.insert(*cursor, c);
                        *cursor += 1;

                        match comp.as_mut() {
                            None => {}
                            Some(cmp) => {
                                *cmp = cmp
                                    .iter()
                                    .filter_map(|i| {
                                        if i.starts_with(&cmd[..*cursor]) {
                                            Some(i[*cursor..].to_string())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>();
                            }
                        }
                    }
                    State::Running(ref mut _pre, ref mut stdin) => {
                        stdin.last_mut().map(|i| i.push(c)).unwrap_or_else(|| {
                            stdin.push(c.to_string());
                        });
                    }
                },
                (KeyCode::Backspace, KeyModifiers::NONE, _) => {
                    self.cursor_backspace();
                }
                (KeyCode::Enter, KeyModifiers::NONE, _) => match self.state {
                    State::Idle(..) => {
                        return self.execute_command();
                    }
                    State::Running(ref mut _pre, ref mut stdin) => {
                        stdin.push(String::new());
                    }
                },
                (KeyCode::Up, KeyModifiers::NONE, _) => {
                    let last = self.history.last().map(|x| x.command.clone());
                    if let Some(last) = last {
                        match self.state {
                            State::Idle(ref mut cmd, ref mut cursor, _) => {
                                *cmd = last;
                                *cursor = cmd.len();
                            }
                            State::Running(..) => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Default::default())
    }

    /// Execute the shell.
    ///
    /// This is the main method that is used to execute the shell. This is where the shell is
    /// created and the input is handled. This also converts the shell into raw mode and enables
    /// the alternate screen.
    ///
    /// This method returns an `anyhow::Result<()>` which is used to handle the errors that are
    /// encountered during the execution of the shell.
    ///
    pub fn execute(mut self) -> anyhow::Result<String> {
        crossterm::terminal::enable_raw_mode()?;

        let mut stdout = io::stdout();
        crossterm::execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let response: anyhow::Result<String> = loop {
            let draw = terminal.draw(|f| self.render(f));

            if let Err(e) = draw {
                break Err(e.into());
            }

            let event = crossterm::event::read();
            let next = match event {
                Ok(event) => self.input(event),
                Err(e) => break Err(e.into()),
            };

            match next {
                Ok(Next::Continue) => continue,
                Ok(Next::Exit(msg)) => break Ok(msg),
                Ok(Next::Clear) => {
                    terminal.clear()?;
                    continue;
                }
                Err(e) => break Err(e),
            }
        };

        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        response
    }

    // helpers

    /// Move the cursor to the left by one.
    fn move_cursor_left(&mut self) {
        match self.state {
            State::Idle(_, 0, _) | State::Running(..) => {}
            State::Idle(_, ref mut cursor, ref mut comp) => {
                *cursor -= 1;
                *comp = None;
            }
        }
    }

    /// Move the cursor to the right by one.
    fn move_cursor_right(&mut self) {
        match self.state {
            State::Idle(ref cmd, cursor, _) if cursor == cmd.len() => {}
            State::Idle(_, ref mut cursor, _) => {
                *cursor += 1;
            }
            State::Running(..) => {}
        }
    }

    /// Move the cursor back by one.
    fn cursor_backspace(&mut self) {
        match self.state {
            State::Idle(ref mut _cmd, 0, _) => {}
            State::Idle(ref mut cmd, ref mut cursor, ref mut comp) => {
                cmd.remove(*cursor - 1);
                *cursor -= 1;
                *comp = None;
            }
            State::Running(ref mut _pre, ref mut stdin) => {
                stdin.last_mut().map(|i| i.pop());
                if stdin.last().map_or(true, |i| i.is_empty()) {
                    stdin.pop();
                }
            }
        }
    }

    /// Continue the execution of the command.
    fn continue_execution(&mut self) -> anyhow::Result<Next> {
        let (prepare, stdin) = match self.state {
            State::Running(ref prep, ref stdin) => (prep.clone(), stdin.clone()),
            State::Idle(..) => return Ok(Next::Continue),
        };

        self._final_execution(&prepare.command, Some(stdin))
    }

    /// Execute the command.
    fn execute_command(&mut self) -> anyhow::Result<Next> {
        let (cmd, _) = match self.state {
            State::Idle(ref cmd, cursor, _) => (cmd.clone(), cursor),
            State::Running(..) => return Ok(Next::Continue),
        };

        let prepare = self.executor.prepare(&cmd);
        self.state = State::Running(prepare.clone(), Vec::new());

        match prepare.stdin_required {
            true => Ok(Next::Continue),
            false => self._final_execution(&cmd, None),
        }
    }

    /// Execute the command and return the next action.
    fn _final_execution(&mut self, cmd: &str, stdin: Option<Vec<String>>) -> anyhow::Result<Next> {
        let prompt = self.executor.prompt(&self.context);
        let output = self.executor.execute(
            &mut self.context,
            command::CommandInput {
                prompt,
                command: cmd.to_string(),
                stdin,
                #[cfg(feature = "tokio")]
                runtime: self.runtime.clone(),
            },
        )?;
        self.state = State::Idle(String::new(), 0, None);

        match output {
            command::OutputAction::Command(command_output) => self.history.push(command_output),
            command::OutputAction::Exit => {
                return Ok(Next::Exit("".to_string()));
            }
            command::OutputAction::Clear => {
                self.history.clear();
                return Ok(Next::Clear);
            }
        }

        Ok(Next::Continue)
    }
}

/// Render the history of the commands.
fn render_history(history: &command::CommandOutput) -> Vec<Line> {
    let command = Line::from(vec![
        Span::styled(history.prompt.clone(), Style::default().blue()),
        Span::raw(" "),
        Span::styled(history.command.clone(), Style::default().bold()),
    ]);
    let stdin = history
        .stdin
        .iter()
        .cloned()
        .map(Span::raw)
        .map(Line::from)
        .collect::<Vec<_>>();
    let stdout = history
        .stdout
        .iter()
        .cloned()
        .map(Span::raw)
        .map(Line::from)
        .collect::<Vec<_>>();
    let stderr = history
        .stderr
        .iter()
        .cloned()
        .map(|i| Span::styled(i, Style::default().red()))
        .map(Line::from)
        .collect::<Vec<_>>();

    let mut lines = vec![command];
    lines.extend(stdin);
    lines.extend(stdout);
    lines.extend(stderr);

    lines
}
