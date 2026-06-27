fn parsear_numero(texto: &str) -> Result<i32, String> {
    match texto.trim().parse::<i32>() {
        Ok(numero) => Ok(numero),
        Err(_) => Err(format!("No se pudo convertir '{texto}' a entero")),
    }
}

fn main() {
    match parsear_numero("42") {
        Ok(n) => println!("Numero: {n}"),
        Err(e) => println!("Error: {e}"),
    }
}
