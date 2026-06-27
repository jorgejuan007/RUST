fn longitud_antigua(texto: &String) -> usize {
    texto.len()
}

fn longitud_idiomatica(texto: &str) -> usize {
    texto.len()
}

fn main() {
    let owned = String::from("rust");
    let borrowed = "prestamo";

    println!("Version antigua con String: {}", longitud_antigua(&owned));
    println!(
        "Version idiomatica con String: {}",
        longitud_idiomatica(&owned)
    );
    println!(
        "Version idiomatica con &str literal: {}",
        longitud_idiomatica(borrowed)
    );
    println!("La firma con &str es mas flexible porque acepta ambos casos.");
}
