use std::fs;
use std::io;
use std::path::Path;

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

/// Los datos del curso viven en `data/`, que esta versionado. Para no modificar el
/// repositorio, este ejemplo trabaja sobre una copia dentro de `target/` (ignorado por git).
fn ruta_de_trabajo() -> io::Result<String> {
    let original = "data/dia_30_tareas.txt";
    let trabajo = "target/dia_30_tareas.txt";

    fs::create_dir_all("target")?;
    if !Path::new(trabajo).exists() && Path::new(original).exists() {
        fs::copy(original, trabajo)?;
    }

    Ok(trabajo.to_string())
}

fn main() -> io::Result<()> {
    let ruta = ruta_de_trabajo()?;
    let mut tareas = cargar_tareas(&ruta)?;

    let nueva = Task {
        titulo: String::from("Cerrar curso de Rust"),
    };

    if !tareas.iter().any(|t| t == &nueva) {
        tareas.push(nueva);
        guardar_tareas(&ruta, &tareas)?;
    }

    for tarea in &tareas {
        println!("{:?}", tarea);
    }

    Ok(())
}
