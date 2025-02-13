fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let app = shelgon::renderer::App::<Executor>::new(rt)?;

    app.execute()
}

pub struct Executor {}

pub struct Context {}

impl shelgon::command::New for Executor {
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized,
    {
        Ok((Self {}, Self::Context {}))
    }
}

impl shelgon::command::Execute for Executor {
    type Context = Context;

    fn prompt(&self, _ctx: &Self::Context) -> String {
        "$".to_string()
    }

    fn prepare(&self, cmd: &str) -> shelgon::command::Prepare {
        if cmd == "cat" {
            return shelgon::command::Prepare {
                command: cmd.to_string(),
                stdin_required: true,
            };
        }

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
