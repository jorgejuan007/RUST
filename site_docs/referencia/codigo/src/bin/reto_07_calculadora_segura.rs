use std::io;

fn leer_linea(prompt: &str) -> Result<String, String> {
    println!("{prompt}");
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .map_err(|e| format!("No se pudo leer la entrada: {e}"))?;
    Ok(entrada.trim().to_string())
}

fn leer_numero(prompt: &str) -> Result<f64, String> {
    let entrada = leer_linea(prompt)?;
    entrada
        .parse::<f64>()
        .map_err(|_| format!("'{entrada}' no es un numero valido"))
}

fn calcular(a: f64, b: f64, op: &str) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("No se puede dividir entre cero".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Operacion invalida: {op}")),
    }
}

fn main() {
    let resultado = (|| -> Result<f64, String> {
        let a = leer_numero("Primer numero:")?;
        let b = leer_numero("Segundo numero:")?;
        let op = leer_linea("Operacion (+, -, *, /):")?;
        calcular(a, b, &op)
    })();

    match resultado {
        Ok(valor) => println!("Resultado: {valor}"),
        Err(error) => println!("Error: {error}"),
    }
}
