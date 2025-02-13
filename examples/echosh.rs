
fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let app = sheller::renderer::App::<Executor>::new(rt)?;

    app.execute()
}

pub struct Executor {}

pub struct Context {}

impl sheller::command::New for Executor {
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized,
    {
        Ok((Self {}, Self::Context {}))
    }
}

impl sheller::command::Execute for Executor {
    type Context = Context;

    fn prompt(&self, _ctx: &Self::Context) -> String {
        "$".to_string()
    }

    fn prepare(&self, cmd: &str) -> sheller::command::Prepare {
        if cmd == "cat" {
            return sheller::command::Prepare {
                command: cmd.to_string(),
                stdin_required: true,
            };
        }

        sheller::command::Prepare {
            command: cmd.to_string(),
            stdin_required: false,
        }
    }

    fn execute(
        &self,
        _ctx: &mut Self::Context,
        cmd: sheller::command::CommandInput,
    ) -> anyhow::Result<sheller::command::OutputAction> {
        let output = sheller::command::CommandOutput {
            prompt: cmd.prompt,
            command: cmd.command.clone(),
            stdin: cmd.stdin.unwrap_or_default(),
            stdout: vec![cmd.command],
            stderr: Vec::new(),
        };
        Ok(sheller::command::OutputAction::Command(output))
    }
}
