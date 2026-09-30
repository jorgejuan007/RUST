use anyhow::{Context, Result};
use clap::Parser;
use rust_30_dias::api::{router, TaskStore};
use std::{net::SocketAddr, path::PathBuf};
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(about = "API persistente de tareas con SQLite", version)]
struct Cli {
    #[arg(long, default_value = "target/tasks.sqlite3")]
    db: PathBuf,
    #[arg(long, default_value = "127.0.0.1:3000")]
    bind: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let store = TaskStore::open(&cli.db).context("no se pudo abrir la base de tareas")?;
    let listener = TcpListener::bind(cli.bind)
        .await
        .context("no se pudo abrir el puerto")?;
    println!("API: http://{}", listener.local_addr()?);
    println!("Base de datos: {}", cli.db.display());
    println!("Pulsa Ctrl+C para detener el servidor.");
    axum::serve(listener, router(store))
        .with_graceful_shutdown(async {
            if let Err(error) = tokio::signal::ctrl_c().await {
                eprintln!("no se pudo escuchar Ctrl+C: {error}");
            }
        })
        .await
        .context("fallo el servidor HTTP")
}
