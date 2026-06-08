enum Comando {
    Agregar(String),
    Completar(u32),
    Salir,
}

fn ejecutar(comando: Comando) {
    match comando {
        Comando::Agregar(texto) if !texto.is_empty() => {
            println!("Agregando tarea: {texto}");
        }
        Comando::Agregar(_) => println!("Titulo vacio"),
        Comando::Completar(id) => println!("Completando tarea {id}"),
        Comando::Salir => println!("Saliendo"),
    }
}

fn main() {
    ejecutar(Comando::Agregar(String::from("Repasar match")));
    ejecutar(Comando::Completar(3));
    ejecutar(Comando::Salir);
}
