struct Task {
    id: u32,
    titulo: String,
}

// Se muestra el bucle antes de introducir Iterator::find en el dia 20.
#[allow(clippy::manual_find)]
fn buscar_tarea(tareas: &[Task], id: u32) -> Option<&Task> {
    for tarea in tareas {
        if tarea.id == id {
            return Some(tarea);
        }
    }
    None
}

fn main() {
    let tareas = vec![
        Task {
            id: 1,
            titulo: String::from("Leer"),
        },
        Task {
            id: 2,
            titulo: String::from("Escribir"),
        },
    ];

    match buscar_tarea(&tareas, 2) {
        Some(tarea) => println!("Encontrada: {}", tarea.titulo),
        None => println!("No existe"),
    }
}
