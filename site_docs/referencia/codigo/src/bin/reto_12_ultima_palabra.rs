fn ultima_palabra(texto: &str) -> &str {
    texto.split_whitespace().last().unwrap_or("")
}

fn main() {
    let frase = "Rust compila ideas seguras";
    println!("Ultima palabra: {}", ultima_palabra(frase));
}
