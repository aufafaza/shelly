use crate::{BuiltIn, Command, Execution};
use anyhow::Context;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::env::set_current_dir;
use std::process::{ChildStdout, Command as ProcessCommand, Stdio};
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
                Ok(commands) => match self.execute_pipeline(commands)? {
                    Execution::Continue => {}
                    Execution::Exit => {
                        self.history.save_history(history_path)?;
                        return Ok(());
                    }
                },

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
                self.last_exit_status = status.code().unwrap_or(1);
                self.jobs.swap_remove(i);
            } else {
                i += 1;
            }
        }
        Ok(())
    }
}

impl Shell {
    pub fn execute_pipeline(&mut self, commands: Vec<Command>) -> anyhow::Result<Execution> {
        let background = matches!(
            commands.last(),
            Some(Command::External {
                background: true,
                ..
            })
        );
        let mut children = Vec::new();
        let mut prev_stdout: Option<ChildStdout> = None;
        let mut commands = commands.into_iter().peekable();
        while let Some(command) = commands.next() {
            let is_last = commands.peek().is_none();
            match command {
                // setup stdin stdout for piping
                Command::BuiltIn(builtin) => match self.execute_builtin(builtin)? {
                    Execution::Exit => return { Ok(Execution::Exit) },
                    Execution::Continue => {}
                },
                // takes stdout output from the previous external command
                Command::External {
                    program,
                    args,
                    background: _,
                } => {
                    let stdin = match prev_stdout.take() {
                        Some(output) => Stdio::from(output),
                        None => Stdio::inherit(),
                    };

                    let stdout = if is_last {
                        Stdio::inherit()
                    } else {
                        Stdio::piped()
                    };

                    let child = ProcessCommand::new(program)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match child {
                        Ok(mut child) => {
                            prev_stdout = child.stdout.take();
                            children.push(child);
                        }
                        Err(e) => {
                            eprintln!("failed to execute {}", e);
                            break;
                        }
                    }
                }
            }
        }
        drop(prev_stdout);
        if background {
            self.jobs.extend(children);
        } else {
            for mut child in children {
                let status = child.wait()?;
                self.last_exit_status = status.code().unwrap_or(1);
            }
        }
        Ok(Execution::Continue)
    }
    pub fn execute_builtin(&mut self, builtin: BuiltIn) -> anyhow::Result<Execution> {
        match builtin {
            BuiltIn::Exit => {
                self.running = false;
                println!("Goodbye");
                Ok(Execution::Exit)
            }
            BuiltIn::Cd(path) => {
                /* match set_current_dir(path) */
                set_current_dir(&path)
                    .with_context(|| format!("cd: cannot change directory {path}"))?;
                Ok(Execution::Continue)
            }
        }
    }
}
