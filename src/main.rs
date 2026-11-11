use shell::execute_command;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::{error::Error, fs::File};

#[tokio::main]
async fn main() {
    let rl = rustyline::DefaultEditor::new().unwrap();
    // move ownership here?
    let user_input_handler = spawn_user_input_handle(rl).await;

    if let Ok(Err(e)) = user_input_handler {
        eprintln!("error: {}", e);
    }
}

pub type CrateResult<T> = anyhow::Result<T>;

pub fn spawn_user_input_handle(
    mut history: rustyline::DefaultEditor,
) -> JoinHandle<CrateResult<()>> {
    let history_path = "/tmp/.shelly_history";

    match history.load_history(history_path) {
        Ok(_) => {}
        Err(ReadlineError::Io(_)) => {
            File::create(history_path).unwrap();
        }
        Err(e) => {
            eprintln!("failed to read or write history file {}", e);
        }
    }

    tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let out = tokio::io::stdout();
        let mut reader = tokio::io::BufReader::new(stdin).lines();
        let mut stdout = tokio::io::BufWriter::new(out);

        stdout.write(b"welcome to shell\n").await?;
        stdout.write(b"\n> ").await?;
        stdout.flush().await?;

        while let Ok(Some(line)) = reader.next_line().await {
            if line == "exit" {
                history.save_history(history_path)?;
                return anyhow::Ok(());
            }
            execute_command(&line);
            history.add_history_entry(line)?;
            stdout.write(b"\n> ").await?;
            stdout.flush().await?;
        }
        Ok(())
    })
}
