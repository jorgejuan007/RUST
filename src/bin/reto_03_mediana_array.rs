fn mediana_ordenado(valores: [i32; 5]) -> i32 {
    valores[2]
}

fn main() {
    let datos = [1, 3, 5, 7, 9];
    println!("Array: {datos:?}");
    println!("Mediana: {}", mediana_ordenado(datos));
}
