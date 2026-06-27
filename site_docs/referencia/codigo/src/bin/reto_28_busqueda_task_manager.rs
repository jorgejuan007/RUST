#[derive(Debug)]
struct Task {
    id: u32,
    titulo: String,
}

struct TaskManager {
    tareas: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        Self { tareas: Vec::new() }
    }

    fn agregar(&mut self, id: u32, titulo: &str) {
        self.tareas.push(Task {
            id,
            titulo: titulo.to_string(),
        });
    }

    fn buscar(&self, id: u32) -> Option<&Task> {
        self.tareas.iter().find(|t| t.id == id)
    }
}

fn main() {
    let mut manager = TaskManager::new();
    manager.agregar(1, "Aprender modulos");
    manager.agregar(2, "Aprender tests");

    match manager.buscar(2) {
        Some(tarea) => println!("Encontrada: {} - {}", tarea.id, tarea.titulo),
        None => println!("No encontrada"),
    }
}
