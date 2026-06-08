enum EstadoTarea {
    Pendiente,
    EnProgreso,
    Hecha,
}

struct Task {
    id: u32,
    titulo: String,
    estado: EstadoTarea,
}

fn describir(estado: &EstadoTarea) -> &'static str {
    match estado {
        EstadoTarea::Pendiente => "Pendiente",
        EstadoTarea::EnProgreso => "En progreso",
        EstadoTarea::Hecha => "Hecha",
    }
}

fn main() {
    let tarea = Task {
        id: 1,
        titulo: String::from("Estudiar enums"),
        estado: EstadoTarea::EnProgreso,
    };

    let _tambien_validos = (EstadoTarea::Pendiente, EstadoTarea::Hecha);

    println!(
        "{} - {} - {}",
        tarea.id,
        tarea.titulo,
        describir(&tarea.estado)
    );
}
