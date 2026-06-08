fn main() {
    let original = String::from("Rust");
    let copia = original.clone();

    println!("Original: {original}");
    println!("Copia: {copia}");

    let movido = original;
    println!("Movido: {movido}");
}
