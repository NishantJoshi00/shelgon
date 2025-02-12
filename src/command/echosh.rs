pub struct Executor {}

pub struct Context {}

impl super::New for Executor {
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized,
    {
        Ok((Self {}, Self::Context {}))
    }
}

impl super::Execute for Executor {
    type Context = Context;

    fn prompt(&self, _ctx: &Self::Context) -> String {
        "$".to_string()
    }

    fn prepare(&self, cmd: &str) -> super::Prepare {
        if cmd == "cat" {
            return super::Prepare {
                command: cmd.to_string(),
                stdin_required: true,
            };
        }

        super::Prepare {
            command: cmd.to_string(),
            stdin_required: false,
        }
    }

    fn execute(
        &self,
        _ctx: &mut Self::Context,
        cmd: super::CommandInput,
    ) -> anyhow::Result<super::CommandOutput> {
        let output = super::CommandOutput {
            prompt: cmd.prompt,
            command: cmd.command.clone(),
            stdin: cmd.stdin.unwrap_or_default(),
            stdout: vec![cmd.command],
            stderr: Vec::new(),
        };
        Ok(output)
    }
}
