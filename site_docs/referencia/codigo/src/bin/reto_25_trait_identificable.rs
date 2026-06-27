trait Identificable {
    fn id(&self) -> u32;
}

struct Usuario {
    id: u32,
    nombre: String,
}

struct Tarea {
    id: u32,
    titulo: String,
}

impl Identificable for Usuario {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Identificable for Tarea {
    fn id(&self) -> u32 {
        self.id
    }
}

fn imprimir_id<T: Identificable>(item: &T) {
    println!("Id: {}", item.id());
}

fn main() {
    let usuario = Usuario {
        id: 7,
        nombre: String::from("Ana"),
    };
    let tarea = Tarea {
        id: 3,
        titulo: String::from("Practicar traits"),
    };

    println!("Usuario: {}", usuario.nombre);
    println!("Tarea: {}", tarea.titulo);
    imprimir_id(&usuario);
    imprimir_id(&tarea);
}
