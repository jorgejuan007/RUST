enum EstadoTarea {
    Pendiente,
    EnProgreso,
    Hecha,
    Bloqueada(String),
}

struct Task {
    titulo: String,
    estado: EstadoTarea,
}

fn describir(estado: &EstadoTarea) -> String {
    match estado {
        EstadoTarea::Pendiente => "Pendiente".to_string(),
        EstadoTarea::EnProgreso => "En progreso".to_string(),
        EstadoTarea::Hecha => "Hecha".to_string(),
        EstadoTarea::Bloqueada(motivo) => format!("Bloqueada: {motivo}"),
    }
}

fn main() {
    let tareas = [
        Task {
            titulo: String::from("Escribir tests"),
            estado: EstadoTarea::Pendiente,
        },
        Task {
            titulo: String::from("Refactorizar"),
            estado: EstadoTarea::EnProgreso,
        },
        Task {
            titulo: String::from("Publicar"),
            estado: EstadoTarea::Hecha,
        },
        Task {
            titulo: String::from("Desplegar"),
            estado: EstadoTarea::Bloqueada(String::from("Falta acceso al servidor")),
        },
    ];

    for tarea in &tareas {
        println!("{} -> {}", tarea.titulo, describir(&tarea.estado));
    }
}
