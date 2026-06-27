use std::collections::HashMap;

fn contar_normalizado(texto: &str) -> HashMap<String, usize> {
    let mut mapa = HashMap::new();

    for palabra in texto.split_whitespace() {
        let clave = palabra.to_lowercase();
        *mapa.entry(clave).or_insert(0) += 1;
    }

    mapa
}

fn main() {
    let texto = "Rust rust RUST seguro Seguro";
    let conteo = contar_normalizado(texto);
    println!("{conteo:?}");
}
