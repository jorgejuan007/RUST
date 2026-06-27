mod parser;
mod stats;

fn main() {
    let texto = "Rust es seguro y rapido";
    let palabras = parser::tokenizar(texto);
    println!("Tokens: {palabras:?}");
    println!("Cantidad: {}", stats::contar_palabras(&palabras));
}
