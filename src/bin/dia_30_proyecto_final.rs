use std::fs;
use std::io;

#[derive(Debug, PartialEq, Eq)]
struct Task {
    titulo: String,
}

fn cargar_tareas(ruta: &str) -> io::Result<Vec<Task>> {
    match fs::read_to_string(ruta) {
        Ok(contenido) => Ok(contenido
            .lines()
            .map(|linea| Task {
                titulo: linea.to_string(),
            })
            .collect()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(error),
    }
}

fn guardar_tareas(ruta: &str, tareas: &[Task]) -> io::Result<()> {
    let contenido = tareas
        .iter()
        .map(|t| t.titulo.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(ruta, contenido)
}

fn main() -> io::Result<()> {
    let ruta = "data/dia_30_tareas.txt";
    let mut tareas = cargar_tareas(ruta)?;

    let nueva = Task {
        titulo: String::from("Cerrar curso de Rust"),
    };

    if !tareas.iter().any(|t| t == &nueva) {
        tareas.push(nueva);
        guardar_tareas(ruta, &tareas)?;
    }

    for tarea in &tareas {
        println!("{:?}", tarea);
    }

    Ok(())
}
