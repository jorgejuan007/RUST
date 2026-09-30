use anyhow::Result;
use clap::{Parser, Subcommand};
use rust_30_dias::api::{
    backup::Backup,
    models::{ListOptions, UpdateTask},
    TaskStore,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "CLI del proyecto final: comparte SQLite con la API", version)]
struct Cli {
    #[arg(long, default_value = "target/tasks.sqlite3")]
    db: PathBuf,
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Add {
        titulo: String,
    },
    List {
        #[arg(long)]
        hecha: Option<bool>,
        #[arg(long)]
        q: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: u32,
        #[arg(long, default_value_t = 0)]
        offset: u32,
    },
    Get {
        id: i64,
    },
    Edit {
        id: i64,
        titulo: String,
    },
    Done {
        id: i64,
    },
    Reopen {
        id: i64,
    },
    Delete {
        id: i64,
    },
    Stats,
    /// Guarda todas las tareas en un archivo JSON nuevo.
    Backup {
        archivo: PathBuf,
    },
    /// Recupera una copia en una base nueva, sin sustituir datos.
    Restore {
        archivo: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let store = TaskStore::open(cli.db)?;
    let output = match cli.command {
        Command::Add { titulo } => serde_json::to_value(store.add(&titulo)?)?,
        Command::List {
            hecha,
            q,
            limit,
            offset,
        } => serde_json::to_value(store.list(&ListOptions {
            hecha,
            q,
            limit: Some(limit),
            offset: Some(offset),
        })?)?,
        Command::Get { id } => serde_json::to_value(store.get(id)?)?,
        Command::Edit { id, titulo } => serde_json::to_value(store.update(
            id,
            &UpdateTask {
                titulo: Some(titulo),
                hecha: None,
            },
        )?)?,
        Command::Done { id } => serde_json::to_value(store.update(
            id,
            &UpdateTask {
                titulo: None,
                hecha: Some(true),
            },
        )?)?,
        Command::Reopen { id } => serde_json::to_value(store.update(
            id,
            &UpdateTask {
                titulo: None,
                hecha: Some(false),
            },
        )?)?,
        Command::Delete { id } => {
            store.delete(id)?;
            serde_json::json!({"borrada": id})
        }
        Command::Stats => serde_json::to_value(store.stats()?)?,
        Command::Backup { archivo } => {
            let backup = store.snapshot()?;
            backup.write_new(&archivo)?;
            serde_json::json!({"archivo": archivo, "tareas": backup.tareas.len(), "ultimo_id": backup.ultimo_id})
        }
        Command::Restore { archivo } => {
            let backup = Backup::read(archivo)?;
            serde_json::to_value(store.restore(&backup)?)?
        }
    };
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
