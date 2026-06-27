struct Task {
    id: u32,
    titulo: String,
    completada: bool,
}

fn buscar_tarea_mut(tareas: &mut [Task], id: u32) -> Option<&mut Task> {
    tareas.iter_mut().find(|t| t.id == id)
}

fn main() {
    let mut tareas = vec![
        Task {
            id: 1,
            titulo: String::from("Leer"),
            completada: false,
        },
        Task {
            id: 2,
            titulo: String::from("Escribir"),
            completada: false,
        },
    ];

    if let Some(tarea) = buscar_tarea_mut(&mut tareas, 2) {
        tarea.completada = true;
        tarea.titulo.push_str(" con cuidado");
    }

    for tarea in &tareas {
        println!("{} - {} - {}", tarea.id, tarea.titulo, tarea.completada);
    }
}
