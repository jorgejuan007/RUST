fn nombre_completo(nombre: &str, apellido: &str) -> String {
    format!("{apellido}, {nombre}")
}

fn main() {
    let completo = nombre_completo("Ada", "Lovelace");
    println!("Nombre formateado: {completo}");
}
