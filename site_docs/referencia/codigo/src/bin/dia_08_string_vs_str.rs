fn construir_saludo(nombre: &str) -> String {
    format!("Hola, {nombre}!")
}

fn main() {
    let nombre = "Lucia";
    let saludo = construir_saludo(nombre);
    println!("{saludo}");
}
