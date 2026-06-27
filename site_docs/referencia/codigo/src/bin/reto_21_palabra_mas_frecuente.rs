use std::collections::HashMap;

fn mas_frecuente(texto: &str) -> Option<(String, usize)> {
    let mut mapa = HashMap::new();

    for palabra in texto.split_whitespace() {
        *mapa.entry(palabra.to_lowercase()).or_insert(0) += 1;
    }

    mapa.into_iter().max_by_key(|(_, cantidad)| *cantidad)
}

fn main() {
    let texto = "rust es seguro y rust es veloz y rust es expresivo";
    println!("Palabra mas frecuente: {:?}", mas_frecuente(texto));
}
