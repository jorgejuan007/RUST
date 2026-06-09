fn imprimir_con_clone(texto: &String) {
    let copia = texto.clone();
    println!("Con clone: {copia}");
}

fn imprimir_con_prestamo(texto: &str) {
    println!("Con prestamo: {texto}");
}

fn main() {
    let mensaje = String::from("Rust evita copias innecesarias");

    imprimir_con_clone(&mensaje);
    println!("Original despues de clone: {mensaje}");

    imprimir_con_prestamo(&mensaje);
    println!("Original despues de prestamo: {mensaje}");
}
