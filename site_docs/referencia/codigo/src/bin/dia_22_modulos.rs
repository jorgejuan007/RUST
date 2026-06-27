mod stats {
    pub fn contar_palabras(texto: &str) -> usize {
        texto.split_whitespace().count()
    }
}

fn main() {
    let texto = "uno dos tres";
    let total = stats::contar_palabras(texto);
    println!("Palabras: {total}");
}
