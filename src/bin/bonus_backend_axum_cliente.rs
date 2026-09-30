use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Parser)]
#[command(about = "Cliente HTTP del backend modular", version)]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:3000")]
    base: String,
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Health,
    List,
    Add { titulo: String },
    Get { id: i64 },
    Done { id: i64 },
    Delete { id: i64 },
    Stats,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let base = cli.base.trim_end_matches('/');
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let request = match cli.command {
        Command::Health => client.get(format!("{base}/salud")),
        Command::List => client.get(format!("{base}/tasks")),
        Command::Add { titulo } => client
            .post(format!("{base}/tasks"))
            .json(&json!({"titulo": titulo})),
        Command::Get { id } => client.get(format!("{base}/tasks/{id}")),
        Command::Done { id } => client
            .patch(format!("{base}/tasks/{id}"))
            .json(&json!({"hecha": true})),
        Command::Delete { id } => client.delete(format!("{base}/tasks/{id}")),
        Command::Stats => client.get(format!("{base}/stats")),
    };
    let response = request
        .send()
        .await
        .context("no se pudo contactar con la API; arranca el servidor primero")?;
    let status = response.status();
    if status == reqwest::StatusCode::NO_CONTENT {
        println!("Tarea borrada.");
        return Ok(());
    }
    let body = response.text().await?;
    anyhow::ensure!(status.is_success(), "HTTP {status}: {body}");
    let value: Value = serde_json::from_str(&body)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
