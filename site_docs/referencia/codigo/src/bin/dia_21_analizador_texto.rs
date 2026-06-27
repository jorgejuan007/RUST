use std::fs;

fn analizar(texto: &str) -> (usize, usize, usize) {
    let lineas = texto.lines().count();
    let palabras = texto.split_whitespace().count();
    let caracteres = texto.chars().count();
    (lineas, palabras, caracteres)
}

fn main() {
    let texto = fs::read_to_string("data/dia_21_texto.txt")
        .unwrap_or_else(|_| String::from("Rust es seguro.\nRust es rapido."));
    let (lineas, palabras, caracteres) = analizar(&texto);

    println!("Lineas: {lineas}");
    println!("Palabras: {palabras}");
    println!("Caracteres: {caracteres}");
}
