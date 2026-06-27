struct Task {
    id: u32,
    titulo: String,
    completada: bool,
}

fn marcar_completada(tareas: &mut [Task], id: u32) {
    for tarea in tareas {
        if tarea.id == id {
            tarea.completada = true;
        }
    }
}

fn main() {
    let mut tareas = vec![
        Task {
            id: 1,
            titulo: String::from("Estudiar Rust"),
            completada: false,
        },
        Task {
            id: 2,
            titulo: String::from("Hacer ejercicio"),
            completada: false,
        },
    ];

    marcar_completada(&mut tareas, 1);

    for tarea in &tareas {
        println!("{} - {} - {}", tarea.id, tarea.titulo, tarea.completada);
    }
}
