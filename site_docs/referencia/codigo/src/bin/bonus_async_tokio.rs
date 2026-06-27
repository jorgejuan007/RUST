use tokio::join;
use tokio::time::{sleep, Duration};

async fn paso(nombre: &str, espera_ms: u64) -> String {
    sleep(Duration::from_millis(espera_ms)).await;
    format!("{nombre} completado en {espera_ms}ms")
}

#[tokio::main]
async fn main() {
    let lectura = paso("lectura de archivo", 120);
    let parseo = paso("parseo de datos", 80);
    let validacion = paso("validacion", 40);

    let (a, b, c) = join!(lectura, parseo, validacion);

    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("Pipeline async finalizado.");
}
