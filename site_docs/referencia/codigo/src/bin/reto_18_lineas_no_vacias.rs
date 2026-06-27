use std::fs;
use std::io;

fn lineas_no_vacias(ruta: &str) -> Result<usize, io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    Ok(contenido.lines().filter(|l| !l.trim().is_empty()).count())
}

fn main() {
    match lineas_no_vacias("data/reto_18_lineas.txt") {
        Ok(total) => println!("Lineas no vacias: {total}"),
        Err(error) => println!("Error al leer archivo: {error}"),
    }
}
