fn imprimir_longitud(texto: &str) {
    println!("Texto: {texto}");
    println!("Longitud: {}", texto.len());
}

fn main() {
    let mensaje = String::from("aprendiendo rust");
    imprimir_longitud(&mensaje);
    println!("El valor sigue disponible: {mensaje}");
}
