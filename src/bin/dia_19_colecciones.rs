use std::collections::HashMap;

fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    let mut conteos = HashMap::new();

    for palabra in texto.split_whitespace() {
        let contador = conteos.entry(palabra.to_string()).or_insert(0);
        *contador += 1;
    }

    conteos
}

fn main() {
    let texto = "rust es rapido y rust es seguro";
    let resultado = contar_palabras(texto);
    println!("{resultado:?}");
}
