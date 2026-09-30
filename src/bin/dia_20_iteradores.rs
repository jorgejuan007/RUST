fn main() {
    let numeros = [1, 2, 3, 4, 5, 6];

    let procesados: Vec<i32> = numeros
        .iter()
        .filter(|n| **n % 2 == 0)
        .map(|n| *n * 10)
        .collect();

    println!("{procesados:?}");
}
