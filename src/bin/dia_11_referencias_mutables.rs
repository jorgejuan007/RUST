fn agregar_mundo(texto: &mut String) {
    texto.push_str(" mundo");
}

fn main() {
    let mut saludo = String::from("hola");
    agregar_mundo(&mut saludo);
    println!("{saludo}");
}
