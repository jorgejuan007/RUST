fn estadisticas(valores: [i32; 5]) -> (i32, f64, i32, i32) {
    let mut suma = 0;
    let mut minimo = valores[0];
    let mut maximo = valores[0];

    for valor in valores {
        suma += valor;
        if valor < minimo {
            minimo = valor;
        }
        if valor > maximo {
            maximo = valor;
        }
    }

    let promedio = suma as f64 / 5.0;
    (suma, promedio, minimo, maximo)
}

fn main() {
    let datos = [7, 3, 9, 1, 6];
    let (suma, promedio, minimo, maximo) = estadisticas(datos);

    println!("Suma: {suma}");
    println!("Promedio: {promedio}");
    println!("Minimo: {minimo}");
    println!("Maximo: {maximo}");
}
