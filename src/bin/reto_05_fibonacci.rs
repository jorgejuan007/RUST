fn fibonacci(cantidad: usize) -> Vec<u64> {
    let mut resultado = Vec::new();

    match cantidad {
        0 => return resultado,
        1 => {
            resultado.push(0);
            return resultado;
        }
        _ => {
            resultado.push(0);
            resultado.push(1);
        }
    }

    while resultado.len() < cantidad {
        let siguiente = resultado[resultado.len() - 1] + resultado[resultado.len() - 2];
        resultado.push(siguiente);
    }

    resultado
}

fn main() {
    let serie = fibonacci(10);
    println!("Primeros 10 numeros de Fibonacci: {serie:?}");
}
