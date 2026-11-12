use crate::{BuiltIn, Command, Execution};
use anyhow::Context;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::env::set_current_dir;
use std::process::Command as ProcessCommand;
use std::{fs::File, process::Child};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
pub struct Shell {
    jobs: Vec<Child>,
    running: bool,
    last_exit_status: i32,
    history: DefaultEditor,
}

impl Shell {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            jobs: Vec::new(),
            running: true,
            last_exit_status: 0,
            history: DefaultEditor::new()?,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let history_path = "/tmp/.shelly_history";

        match self.history.load_history(history_path) {
            Ok(_) => {}
            Err(ReadlineError::Io(_)) => {
                File::create(history_path)?;
            }
            Err(e) => {
                eprintln!("failed to read or write history file {}", e);
            }
        }

        let stdin = tokio::io::stdin();
        let out = tokio::io::stdout();
        let mut reader = tokio::io::BufReader::new(stdin).lines();
        let mut stdout = tokio::io::BufWriter::new(out);

        stdout.write(b"welcome to shell\n").await?;
        stdout.write(b"\n> ").await?;
        stdout.flush().await?;

        while let Ok(Some(line)) = reader.next_line().await {
            // clean jobs
            self.collect_and_kill_jobs().await?;
            match Command::parse(&line) {
                Ok(commands) => {
                    self.execute_pipeline(commands)?;
                }
                Err(e) => eprintln!("{e:?}"),
            }
            self.history.add_history_entry(line)?;
            stdout.write(b"\n> ").await?;
            stdout.flush().await?;
        }
        Ok(())
    }

    pub async fn collect_and_kill_jobs(&mut self) -> anyhow::Result<()> {
        let mut i = 0;
        while i < self.jobs.len() {
            if let Some(status) = self.jobs[i].try_wait()? {
                println!("PID {} finished: {}", self.jobs[i].id(), status);
                self.last_exit_status = status.code().unwrap();
                self.jobs.swap_remove(i);
            } else {
                i += 1;
            }
        }
        Ok(())
    }
}

impl Shell {
    pub fn execute_pipeline(&mut self, commands: Vec<Command>) -> anyhow::Result<Execution> {}
    pub fn execute(&mut self, command: Command) -> anyhow::Result<Execution> {
        match command {
            Command::External {
                program,
                args,
                background,
            } => {
                let child = ProcessCommand::new(&program).args(&args).spawn()?;
                println!("Child pID: {}", child.id());
                println!("doing other work");
                if background {
                    self.jobs.push(child);
                } else {
                    let mut child = child;
                    child.wait()?;
                }
                Ok(Execution::Continue)
            }
            Command::BuiltIn(BuiltIn::Exit) => {
                self.running = false;
                Ok(Execution::Exit)
            }
            Command::BuiltIn(BuiltIn::Cd(path)) => {
                /* match set_current_dir(path) */
                set_current_dir(&path)
                    .with_context(|| format!("cd: cannot change directory {path}"))?;
                Ok(Execution::Continue)
            }
        }
    }
}
