use clap::{Parser, Subcommand};
use rust_30_dias::tareas_json::{Estado, TaskStore, TaskStoreError};
use std::fs;
use std::path::{Path, PathBuf};

const RUTA_ORIGINAL: &str = "data/bonus_tareas.json";
const RUTA_TRABAJO: &str = "target/bonus_tareas.json";

#[derive(Debug, Parser)]
#[command(
    name = "bonus_gestor_tareas_json",
    version,
    about = "Gestor de tareas con clap, serde y persistencia JSON"
)]
struct Cli {
    #[command(subcommand)]
    comando: Comando,
}

#[derive(Debug, Subcommand)]
enum Comando {
    List,
    Add { titulo: Vec<String> },
    Done { id: u32 },
    Delete { id: u32 },
    Search { termino: Vec<String> },
}

fn imprimir_tareas(tareas: &[rust_30_dias::tareas_json::Task]) {
    if tareas.is_empty() {
        println!("No hay tareas.");
        return;
    }

    for tarea in tareas {
        let estado = match tarea.estado {
            Estado::Pendiente => "[ ]",
            Estado::EnProgreso => "[~]",
            Estado::Hecha => "[x]",
        };
        println!("{estado} {} - {}", tarea.id, tarea.titulo);
    }
}

/// La lista inicial esta en `data/`, versionado en el repositorio. Se trabaja sobre una
/// copia en `target/` (ignorado por git) para no modificar los datos del curso.
fn ruta_de_trabajo() -> std::io::Result<PathBuf> {
    fs::create_dir_all("target")?;
    if !Path::new(RUTA_TRABAJO).exists() && Path::new(RUTA_ORIGINAL).exists() {
        fs::copy(RUTA_ORIGINAL, RUTA_TRABAJO)?;
    }

    Ok(PathBuf::from(RUTA_TRABAJO))
}

fn main() -> Result<(), TaskStoreError> {
    let cli = Cli::parse();
    let ruta = ruta_de_trabajo()?;
    let mut store = TaskStore::load(&ruta)?;

    match cli.comando {
        Comando::List => {
            imprimir_tareas(store.list());
        }
        Comando::Add { titulo } => {
            let titulo = titulo.join(" ");
            if titulo.trim().is_empty() {
                println!("Debes indicar un titulo no vacio.");
                return Ok(());
            }
            let id = store.add(&titulo);
            store.save(&ruta)?;
            println!("Tarea agregada con id {id}.");
        }
        Comando::Done { id } => {
            if store.complete(id) {
                store.save(&ruta)?;
                println!("Tarea completada.");
            } else {
                println!("No existe una tarea con ese id.");
            }
        }
        Comando::Delete { id } => {
            if store.delete(id) {
                store.save(&ruta)?;
                println!("Tarea eliminada.");
            } else {
                println!("No existe una tarea con ese id.");
            }
        }
        Comando::Search { termino } => {
            let termino = termino.join(" ");
            if termino.trim().is_empty() {
                println!("Debes indicar un termino de busqueda.");
                return Ok(());
            }
            let resultados = store.search(&termino);
            if resultados.is_empty() {
                println!("No hay coincidencias.");
            } else {
                for tarea in resultados {
                    println!("#{} - {}", tarea.id, tarea.titulo);
                }
            }
        }
    }

    Ok(())
}
