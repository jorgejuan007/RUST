fn parsear_entero(texto: &str) -> Result<i32, String> {
    texto
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("No se pudo parsear '{texto}'"))
}

fn mitad_si_es_par(texto: &str) -> Result<Option<i32>, String> {
    parsear_entero(texto).map(|n| if n % 2 == 0 { Some(n / 2) } else { None })
}

fn main() {
    let valor = "40";
    let resultado = mitad_si_es_par(valor)
        .map(|opcion| opcion.unwrap_or(-1))
        .unwrap_or_else(|error| {
            println!("Error: {error}");
            -999
        });

    let mensaje = Some("rust")
        .map(|texto| texto.to_uppercase())
        .unwrap_or_else(|| String::from("VACIO"));

    println!("Resultado procesado: {resultado}");
    println!("Mensaje: {mensaje}");
}
