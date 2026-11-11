use shell::Shell;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("error: {}", error);
    }
}

async fn run() -> anyhow::Result<()> {
    let mut shell = Shell::new()?;
    shell.run().await
}
