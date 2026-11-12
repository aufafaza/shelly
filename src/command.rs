use anyhow::bail;

#[derive(Debug)]
pub enum Command {
    BuiltIn(BuiltIn),
    External {
        program: String,
        args: Vec<String>,
        background: bool,
    },
}

#[derive(Debug)]
pub enum BuiltIn {
    Cd(String),
    Exit,
}

pub enum Execution {
    Continue,
    Exit,
}

impl Command {
    pub fn parse(input: &str) -> anyhow::Result<Vec<Self>> {
        if input.trim().is_empty() {
            return Ok(Vec::new());
        }
        let mut commands = Vec::new();
        for segment in input.split(" | ").peekable() {
            let mut parts: Vec<&str> = segment.split_whitespace().collect();
            if parts.is_empty() {
                bail!("expected commands after |");
            }
            let background = if parts.last() == Some(&"&") {
                parts.pop();
                true
            } else {
                false
            };
            let command = match parts.as_slice() {
                [] => bail!("expected a command"),
                ["exit"] => Command::BuiltIn(BuiltIn::Exit),
                ["exit", ..] => bail!("exit: expected no arguments"),
                ["cd", path] => Command::BuiltIn(BuiltIn::Cd((*path).to_owned())),
                ["cd", ..] => bail!("cd: expected a dir"),
                [program, args @ ..] => Command::External {
                    program: (*program).to_owned(),
                    args: args.iter().map(|arg| (*arg).to_owned()).collect(),
                    background,
                },
            };
            commands.push(command);
        }
        Ok(commands)
    }
}
