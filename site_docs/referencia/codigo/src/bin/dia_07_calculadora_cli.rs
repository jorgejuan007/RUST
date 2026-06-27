use std::error::Error;
use std::io;

fn leer_linea(prompt: &str) -> Result<String, io::Error> {
    println!("{prompt}");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada)?;
    Ok(entrada.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let a: f64 = leer_linea("Primer numero:")?.parse()?;
    let b: f64 = leer_linea("Segundo numero:")?.parse()?;
    let op = leer_linea("Operacion (+, -, *, /):")?;

    let resultado = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => {
            println!("Operacion invalida");
            return Ok(());
        }
    };

    println!("Resultado: {resultado}");
    Ok(())
}
