use shell::execute_command;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};

#[tokio::main]
async fn main() {
    let user_input_handler = spawn_user_input_handle().await;

    if let Ok(Err(e)) = user_input_handler {
        eprintln!("error: {}", e);
    }
}

pub type CrateResult<T> = anyhow::Result<T>;

pub fn spawn_user_input_handle() -> JoinHandle<CrateResult<()>> {
    tokio::spawn(async {
        let stdin = tokio::io::stdin();
        let out = tokio::io::stdout();
        let mut reader = tokio::io::BufReader::new(stdin).lines();
        let mut stdout = tokio::io::BufWriter::new(out);

        stdout.write(b"welcome to shell\n").await?;
        stdout.write(b"\n>").await?;
        stdout.flush().await?;

        while let Ok(Some(line)) = reader.next_line().await {
            execute_command(&line);
            stdout.write(b"\n>").await?;
            stdout.flush().await?;
        }
        Ok(())
    })
}
