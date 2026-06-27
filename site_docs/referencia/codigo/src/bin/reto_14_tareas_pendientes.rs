struct Task {
    id: u32,
    titulo: String,
    completada: bool,
}

fn pendientes(tareas: &[Task]) -> usize {
    tareas.iter().filter(|t| !t.completada).count()
}

fn main() {
    let tareas = vec![
        Task {
            id: 1,
            titulo: String::from("Estudiar"),
            completada: false,
        },
        Task {
            id: 2,
            titulo: String::from("Practicar"),
            completada: true,
        },
        Task {
            id: 3,
            titulo: String::from("Revisar"),
            completada: false,
        },
    ];

    for tarea in &tareas {
        println!("{} - {} - {}", tarea.id, tarea.titulo, tarea.completada);
    }
    println!("Pendientes: {}", pendientes(&tareas));
}
