use std::env;
use std::fs;
use std::io;

const RUTA: &str = "data/reto_30_tareas_cli.tsv";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Estado {
    Pendiente,
    Hecha,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Task {
    id: u32,
    estado: Estado,
    titulo: String,
}

fn parsear_linea(linea: &str) -> Option<Task> {
    let mut partes = linea.splitn(3, '\t');
    let id = partes.next()?.parse().ok()?;
    let estado = match partes.next()? {
        "pendiente" => Estado::Pendiente,
        "hecha" => Estado::Hecha,
        _ => return None,
    };
    let titulo = partes.next()?.to_string();
    Some(Task { id, estado, titulo })
}

fn serializar_tarea(tarea: &Task) -> String {
    let estado = match tarea.estado {
        Estado::Pendiente => "pendiente",
        Estado::Hecha => "hecha",
    };
    format!("{}\t{}\t{}", tarea.id, estado, tarea.titulo)
}

fn cargar_tareas(ruta: &str) -> io::Result<Vec<Task>> {
    match fs::read_to_string(ruta) {
        Ok(contenido) => Ok(contenido.lines().filter_map(parsear_linea).collect()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error),
    }
}

fn guardar_tareas(ruta: &str, tareas: &[Task]) -> io::Result<()> {
    let contenido = tareas
        .iter()
        .map(serializar_tarea)
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(ruta, contenido)
}

fn siguiente_id(tareas: &[Task]) -> u32 {
    tareas.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn ayuda() {
    println!("Uso:");
    println!("cargo run --bin reto_30_cli_tareas -- list");
    println!("cargo run --bin reto_30_cli_tareas -- add Comprar pan");
    println!("cargo run --bin reto_30_cli_tareas -- done 1");
    println!("cargo run --bin reto_30_cli_tareas -- delete 1");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut tareas = cargar_tareas(RUTA)?;

    if args.is_empty() {
        ayuda();
        return Ok(());
    }

    match args[0].as_str() {
        "list" => {
            for tarea in &tareas {
                let estado = match tarea.estado {
                    Estado::Pendiente => "[ ]",
                    Estado::Hecha => "[x]",
                };
                println!("{estado} {} - {}", tarea.id, tarea.titulo);
            }
        }
        "add" => {
            let titulo = args[1..].join(" ");
            if titulo.trim().is_empty() {
                println!("Debes indicar un titulo.");
                return Ok(());
            }
            tareas.push(Task {
                id: siguiente_id(&tareas),
                estado: Estado::Pendiente,
                titulo,
            });
            guardar_tareas(RUTA, &tareas)?;
            println!("Tarea agregada.");
        }
        "done" => {
            let Some(id_texto) = args.get(1) else {
                println!("Debes indicar un id.");
                return Ok(());
            };
            let Ok(id) = id_texto.parse::<u32>() else {
                println!("El id debe ser numerico.");
                return Ok(());
            };
            if let Some(tarea) = tareas.iter_mut().find(|t| t.id == id) {
                tarea.estado = Estado::Hecha;
                guardar_tareas(RUTA, &tareas)?;
                println!("Tarea completada.");
            } else {
                println!("No existe una tarea con ese id.");
            }
        }
        "delete" => {
            let Some(id_texto) = args.get(1) else {
                println!("Debes indicar un id.");
                return Ok(());
            };
            let Ok(id) = id_texto.parse::<u32>() else {
                println!("El id debe ser numerico.");
                return Ok(());
            };
            let len_inicial = tareas.len();
            tareas.retain(|t| t.id != id);
            if tareas.len() != len_inicial {
                guardar_tareas(RUTA, &tareas)?;
                println!("Tarea eliminada.");
            } else {
                println!("No existe una tarea con ese id.");
            }
        }
        _ => ayuda(),
    }

    Ok(())
}
