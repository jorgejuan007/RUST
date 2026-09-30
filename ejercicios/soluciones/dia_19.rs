pub fn frecuencias(texto: &str) -> std::collections::HashMap<String, usize> {
    let mut mapa = std::collections::HashMap::new();
    for palabra in texto.split_whitespace() {
        *mapa.entry(palabra.to_lowercase()).or_insert(0) += 1;
    }
    mapa
}
