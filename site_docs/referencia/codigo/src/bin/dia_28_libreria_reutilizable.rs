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

    fn total(&self) -> usize {
        self.tareas.len()
    }
}

fn main() {
    let mut manager = TaskManager::new();
    manager.agregar(1, "Refactorizar");
    manager.agregar(2, "Probar");
    println!("Total: {}", manager.total());

    for tarea in &manager.tareas {
        println!("{} - {}", tarea.id, tarea.titulo);
    }
}
