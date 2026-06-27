#[derive(Debug)]
enum ErrorParseo {
    CadenaVacia,
    NoEsEntero(String),
}

fn parsear_numero(texto: &str) -> Result<i32, ErrorParseo> {
    let limpio = texto.trim();

    if limpio.is_empty() {
        return Err(ErrorParseo::CadenaVacia);
    }

    limpio
        .parse::<i32>()
        .map_err(|_| ErrorParseo::NoEsEntero(limpio.to_string()))
}

fn describir_error(error: &ErrorParseo) -> String {
    match error {
        ErrorParseo::CadenaVacia => "La cadena esta vacia".to_string(),
        ErrorParseo::NoEsEntero(valor) => format!("'{valor}' no contiene un entero valido"),
    }
}

fn main() {
    for caso in ["42", "", "hola"] {
        match parsear_numero(caso) {
            Ok(numero) => println!("'{caso}' -> {numero}"),
            Err(error) => println!("'{caso}' -> {}", describir_error(&error)),
        }
    }
}
