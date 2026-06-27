use tokio::task::JoinSet;
use tokio::time::{sleep, timeout, Duration};

async fn descargar(id: u8, espera_ms: u64) -> String {
    sleep(Duration::from_millis(espera_ms)).await;
    format!("recurso #{id} listo en {espera_ms}ms")
}

async fn procesar(nombre: &str, espera_ms: u64) -> String {
    sleep(Duration::from_millis(espera_ms)).await;
    format!("{nombre} procesado en {espera_ms}ms")
}

#[tokio::main]
async fn main() {
    println!("Descargas concurrentes con spawn y JoinSet:");
    let mut tareas = JoinSet::new();

    for (id, espera_ms) in [(1, 90), (2, 40), (3, 120)] {
        tareas.spawn(descargar(id, espera_ms));
    }

    while let Some(resultado) = tareas.join_next().await {
        match resultado {
            Ok(mensaje) => println!("- {mensaje}"),
            Err(error) => println!("- tarea fallida: {error}"),
        }
    }

    println!();
    println!("Timeout defensivo:");
    match timeout(Duration::from_millis(100), procesar("informe", 140)).await {
        Ok(mensaje) => println!("- {mensaje}"),
        Err(_) => println!("- timeout al procesar informe"),
    }

    println!();
    println!("select! para reaccionar al evento que llegue primero:");
    let trabajo = procesar("resumen", 60);
    tokio::pin!(trabajo);

    let watchdog = sleep(Duration::from_millis(90));
    tokio::pin!(watchdog);

    tokio::select! {
        resultado = &mut trabajo => {
            println!("- completado antes del watchdog: {resultado}");
        }
        _ = &mut watchdog => {
            println!("- watchdog activado antes de terminar");
        }
    }
}
