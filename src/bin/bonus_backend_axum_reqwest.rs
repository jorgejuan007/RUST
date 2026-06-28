use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Task {
    id: u32,
    titulo: String,
    hecha: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewTask {
    titulo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Salud {
    servicio: String,
    estado: String,
}

#[derive(Clone)]
struct AppState {
    tareas: Arc<Mutex<Vec<Task>>>,
}

impl AppState {
    fn seed() -> Self {
        Self {
            tareas: Arc::new(Mutex::new(vec![
                Task {
                    id: 1,
                    titulo: "Repasar Result".to_string(),
                    hecha: true,
                },
                Task {
                    id: 2,
                    titulo: "Montar API local".to_string(),
                    hecha: false,
                },
            ])),
        }
    }

    fn list(&self) -> Vec<Task> {
        self.tareas.lock().expect("mutex envenenado").clone()
    }

    fn get(&self, id: u32) -> Option<Task> {
        self.tareas
            .lock()
            .expect("mutex envenenado")
            .iter()
            .find(|task| task.id == id)
            .cloned()
    }

    fn add(&self, titulo: String) -> Task {
        let mut tareas = self.tareas.lock().expect("mutex envenenado");
        let id = tareas.last().map(|task| task.id + 1).unwrap_or(1);

        let task = Task {
            id,
            titulo,
            hecha: false,
        };
        tareas.push(task.clone());
        task
    }
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/salud", get(salud))
        .route("/tasks", get(listar_tareas).post(crear_tarea))
        .route("/tasks/{id}", get(ver_tarea))
        .with_state(state)
}

async fn salud() -> Json<Salud> {
    Json(Salud {
        servicio: "curso-rust-local".to_string(),
        estado: "ok".to_string(),
    })
}

async fn listar_tareas(State(state): State<AppState>) -> Json<Vec<Task>> {
    Json(state.list())
}

async fn ver_tarea(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<Task>, StatusCode> {
    state.get(id).map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn crear_tarea(
    State(state): State<AppState>,
    Json(payload): Json<NewTask>,
) -> (StatusCode, Json<Task>) {
    let task = state.add(payload.titulo);
    (StatusCode::CREATED, Json(task))
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState::seed();
    let app = build_app(state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("no se pudo abrir el puerto local")?;
    let direccion = listener
        .local_addr()
        .context("no se pudo leer la direccion local")?;

    let servidor = tokio::spawn(async move {
        if let Err(error) = axum::serve(listener, app).await {
            eprintln!("servidor detenido con error: {error}");
        }
    });

    sleep(Duration::from_millis(50)).await;

    let base = format!("http://{direccion}");
    let client = Client::new();

    let salud: Salud = client
        .get(format!("{base}/salud"))
        .send()
        .await
        .context("fallo la llamada a /salud")?
        .error_for_status()
        .context("respuesta HTTP no valida en /salud")?
        .json()
        .await
        .context("no se pudo parsear la respuesta de /salud")?;

    let creada: Task = client
        .post(format!("{base}/tasks"))
        .json(&NewTask {
            titulo: "Probar reqwest contra axum".to_string(),
        })
        .send()
        .await
        .context("fallo la llamada POST /tasks")?
        .error_for_status()
        .context("respuesta HTTP no valida en POST /tasks")?
        .json()
        .await
        .context("no se pudo parsear la respuesta de POST /tasks")?;

    let tareas: Vec<Task> = client
        .get(format!("{base}/tasks"))
        .send()
        .await
        .context("fallo la llamada GET /tasks")?
        .error_for_status()
        .context("respuesta HTTP no valida en GET /tasks")?
        .json()
        .await
        .context("no se pudo parsear la respuesta de GET /tasks")?;

    let recuperada: Task = client
        .get(format!("{base}/tasks/{}", creada.id))
        .send()
        .await
        .context("fallo la llamada GET /tasks/{id}")?
        .error_for_status()
        .context("respuesta HTTP no valida en GET /tasks/{id}")?
        .json()
        .await
        .context("no se pudo parsear la respuesta de GET /tasks/{id}")?;

    println!("Servicio: {} ({})", salud.servicio, salud.estado);
    println!("Tarea creada: #{} - {}", creada.id, creada.titulo);
    println!("Total de tareas actuales: {}", tareas.len());
    println!(
        "Consulta puntual: #{} - {}",
        recuperada.id, recuperada.titulo
    );

    servidor.abort();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::AppState;

    #[test]
    fn agrega_tareas_con_ids_incrementales() {
        let state = AppState::seed();

        let task = state.add("Nueva tarea".to_string());

        assert_eq!(task.id, 3);
        assert_eq!(state.list().len(), 3);
    }

    #[test]
    fn recupera_tarea_existente() {
        let state = AppState::seed();

        let task = state.get(2).expect("deberia existir la tarea 2");

        assert_eq!(task.titulo, "Montar API local");
        assert!(!task.hecha);
    }
}
