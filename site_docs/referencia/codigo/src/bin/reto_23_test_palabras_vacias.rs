fn contar_palabras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

fn main() {
    println!("Palabras en cadena vacia: {}", contar_palabras(""));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cadena_vacia_tiene_cero_palabras() {
        assert_eq!(contar_palabras(""), 0);
    }
}
