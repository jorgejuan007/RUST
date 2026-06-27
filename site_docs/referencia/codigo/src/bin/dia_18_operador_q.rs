use std::fs;
use std::io;

fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    Ok(contenido)
}

fn main() {
    match leer_archivo("data/dia_18_datos.txt") {
        Ok(texto) => println!("{texto}"),
        Err(e) => println!("Error al leer archivo: {e}"),
    }
}
