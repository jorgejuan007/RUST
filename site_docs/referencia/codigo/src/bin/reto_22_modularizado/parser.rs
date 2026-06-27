pub fn tokenizar(texto: &str) -> Vec<&str> {
    texto.split_whitespace().collect()
}
