pub struct Executor {}

pub struct Context {}

impl crate::command::New for Executor {
    fn new() -> anyhow::Result<(Self, Self::Context)>
    where
        Self: Sized,
    {
        Ok((Self {}, Self::Context {}))
    }
}

impl crate::command::Execute for Executor {
    type Context = Context;

    fn prompt(&self, _ctx: &Self::Context) -> String {
        "$".to_string()
    }

    fn prepare(&self, cmd: &str) -> crate::command::Prepare {
        if cmd == "cat" {
            return crate::command::Prepare {
                command: cmd.to_string(),
                stdin_required: true,
            };
        }

        crate::command::Prepare {
            command: cmd.to_string(),
            stdin_required: false,
        }
    }

    fn execute(
        &self,
        _ctx: &mut Self::Context,
        cmd: crate::command::CommandInput,
    ) -> anyhow::Result<crate::command::OutputAction> {
        let output = crate::command::CommandOutput {
            prompt: cmd.prompt,
            command: cmd.command.clone(),
            stdin: cmd.stdin.unwrap_or_default(),
            stdout: vec![cmd.command],
            stderr: Vec::new(),
        };
        Ok(crate::command::OutputAction::Command(output))
    }
}
