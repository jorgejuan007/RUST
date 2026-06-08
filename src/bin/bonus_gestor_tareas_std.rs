use std::env;
use std::fs;
use std::io;

const RUTA_TAREAS: &str = "data/bonus_tareas.tsv";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Estado {
    Pendiente,
    Hecha,
}

impl Estado {
    fn desde_texto(texto: &str) -> Option<Self> {
        match texto {
            "pendiente" => Some(Self::Pendiente),
            "hecha" => Some(Self::Hecha),
            _ => None,
        }
    }

    fn como_texto(&self) -> &'static str {
        match self {
            Self::Pendiente => "pendiente",
            Self::Hecha => "hecha",
        }
    }
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
    let estado = Estado::desde_texto(partes.next()?)?;
    let titulo = partes.next()?.to_string();

    Some(Task { id, estado, titulo })
}

fn serializar_tarea(tarea: &Task) -> String {
    format!(
        "{}\t{}\t{}",
        tarea.id,
        tarea.estado.como_texto(),
        tarea.titulo
    )
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

fn listar_tareas(tareas: &[Task]) {
    if tareas.is_empty() {
        println!("No hay tareas.");
        return;
    }

    for tarea in tareas {
        let estado = match tarea.estado {
            Estado::Pendiente => "[ ]",
            Estado::Hecha => "[x]",
        };
        println!("{estado} {} - {}", tarea.id, tarea.titulo);
    }
}

fn agregar_tarea(tareas: &mut Vec<Task>, titulo: String) {
    let id = siguiente_id(tareas);
    tareas.push(Task {
        id,
        estado: Estado::Pendiente,
        titulo,
    });
}

fn completar_tarea(tareas: &mut [Task], id: u32) -> bool {
    for tarea in tareas {
        if tarea.id == id {
            tarea.estado = Estado::Hecha;
            return true;
        }
    }
    false
}

fn borrar_tarea(tareas: &mut Vec<Task>, id: u32) -> bool {
    let longitud_inicial = tareas.len();
    tareas.retain(|t| t.id != id);
    tareas.len() != longitud_inicial
}

fn ayuda() {
    println!("Uso:");
    println!("cargo run --bin bonus_gestor_tareas_std -- list");
    println!("cargo run --bin bonus_gestor_tareas_std -- add Comprar leche");
    println!("cargo run --bin bonus_gestor_tareas_std -- done 1");
    println!("cargo run --bin bonus_gestor_tareas_std -- delete 1");
}

fn main() -> io::Result<()> {
    let mut tareas = cargar_tareas(RUTA_TAREAS)?;
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        ayuda();
        return Ok(());
    }

    match args[0].as_str() {
        "list" => {
            listar_tareas(&tareas);
        }
        "add" => {
            let titulo = args[1..].join(" ");
            if titulo.trim().is_empty() {
                println!("Debes indicar un titulo.");
                return Ok(());
            }
            agregar_tarea(&mut tareas, titulo);
            guardar_tareas(RUTA_TAREAS, &tareas)?;
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
            if completar_tarea(&mut tareas, id) {
                guardar_tareas(RUTA_TAREAS, &tareas)?;
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
            if borrar_tarea(&mut tareas, id) {
                guardar_tareas(RUTA_TAREAS, &tareas)?;
                println!("Tarea eliminada.");
            } else {
                println!("No existe una tarea con ese id.");
            }
        }
        _ => ayuda(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsea_linea_valida() {
        let tarea = parsear_linea("3\tpendiente\tEstudiar").unwrap();
        assert_eq!(tarea.id, 3);
        assert_eq!(tarea.estado, Estado::Pendiente);
        assert_eq!(tarea.titulo, "Estudiar");
    }

    #[test]
    fn calcula_siguiente_id() {
        let tareas = vec![
            Task {
                id: 2,
                estado: Estado::Pendiente,
                titulo: String::from("A"),
            },
            Task {
                id: 7,
                estado: Estado::Hecha,
                titulo: String::from("B"),
            },
        ];

        assert_eq!(siguiente_id(&tareas), 8);
    }
}
