pub fn primera_palabra(texto: &str) -> &str {
    texto.split_whitespace().next().unwrap_or("")
}
