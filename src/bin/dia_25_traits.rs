trait Resumen {
    fn resumen(&self) -> String;
}

struct Libro {
    titulo: String,
}

struct Tarea {
    titulo: String,
}

impl Resumen for Libro {
    fn resumen(&self) -> String {
        format!("Libro: {}", self.titulo)
    }
}

impl Resumen for Tarea {
    fn resumen(&self) -> String {
        format!("Tarea: {}", self.titulo)
    }
}

fn imprimir_resumen<T: Resumen>(item: &T) {
    println!("{}", item.resumen());
}

fn main() {
    let libro = Libro {
        titulo: String::from("The Book"),
    };
    let tarea = Tarea {
        titulo: String::from("Practicar traits"),
    };

    imprimir_resumen(&libro);
    imprimir_resumen(&tarea);
}
