fn es_par(n: i32) -> bool {
    n % 2 == 0
}

fn primera_palabra(texto: &str) -> &str {
    match texto.split_whitespace().next() {
        Some(palabra) => palabra,
        None => "",
    }
}

fn main() {
    println!("8 es par: {}", es_par(8));
    println!("Primera palabra: {}", primera_palabra("rust es genial"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_es_par() {
        assert!(es_par(8));
        assert!(!es_par(5));
    }

    #[test]
    fn prueba_primera_palabra() {
        assert_eq!(primera_palabra("rust es genial"), "rust");
        assert_eq!(primera_palabra(""), "");
    }
}
