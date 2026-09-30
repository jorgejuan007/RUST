pub fn normalizar(texto: &mut String) {
    *texto = texto
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
}
