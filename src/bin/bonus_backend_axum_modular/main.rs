mod handlers;
mod models;
mod state;
mod storage;

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use handlers::{crear_tarea, listar_tareas, obtener_estadisticas, obtener_tarea, salud};
use models::{HealthResponse, NewTask, Task, TaskStats};
use reqwest::Client;
use state::AppState;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/salud", get(salud))
        .route("/stats", get(obtener_estadisticas))
        .route("/tasks", get(listar_tareas).post(crear_tarea))
        .route("/tasks/{id}", get(obtener_tarea))
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState::new();
    let app = build_app(state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("no se pudo abrir el puerto local")?;
    let address = listener
        .local_addr()
        .context("no se pudo leer la direccion local")?;

    let server = tokio::spawn(async move {
        if let Err(error) = axum::serve(listener, app).await {
            eprintln!("servidor modular detenido con error: {error}");
        }
    });

    sleep(Duration::from_millis(50)).await;

    let base = format!("http://{address}");
    let client = Client::new();

    let health: HealthResponse = client
        .get(format!("{base}/salud"))
        .send()
        .await
        .context("fallo la llamada a /salud")?
        .error_for_status()
        .context("respuesta HTTP no valida en /salud")?
        .json()
        .await
        .context("no se pudo parsear la respuesta de /salud")?;

    let created: Task = client
        .post(format!("{base}/tasks"))
        .json(&NewTask {
            titulo: "Separar handlers y storage".to_string(),
        })
        .send()
        .await
        .context("fallo la llamada POST /tasks")?
        .error_for_status()
        .context("respuesta HTTP no valida en POST /tasks")?
        .json()
        .await
        .context("no se pudo parsear la respuesta de POST /tasks")?;

    let tasks: Vec<Task> = client
        .get(format!("{base}/tasks"))
        .send()
        .await
        .context("fallo la llamada GET /tasks")?
        .error_for_status()
        .context("respuesta HTTP no valida en GET /tasks")?
        .json()
        .await
        .context("no se pudo parsear la respuesta de GET /tasks")?;

    let stats: TaskStats = client
        .get(format!("{base}/stats"))
        .send()
        .await
        .context("fallo la llamada GET /stats")?
        .error_for_status()
        .context("respuesta HTTP no valida en GET /stats")?
        .json()
        .await
        .context("no se pudo parsear la respuesta de GET /stats")?;

    println!("Servicio modular: {} ({})", health.servicio, health.estado);
    println!("Tarea creada: #{} - {}", created.id, created.titulo);
    println!("Tareas visibles: {}", tasks.len());
    println!(
        "Stats: total={}, pendientes={}",
        stats.total, stats.pendientes
    );

    server.abort();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::storage::TaskStore;

    #[test]
    fn storage_seed_y_add_funcionan() {
        let store = TaskStore::seed();

        let created = store.add("Nueva tarea modular".to_string()).unwrap();
        let tasks = store.list().unwrap();

        assert_eq!(created.id, 3);
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn storage_stats_refleja_pendientes() {
        let store = TaskStore::seed();
        let stats = store.stats().unwrap();

        assert_eq!(stats.total, 2);
        assert_eq!(stats.pendientes, 1);
    }
}
