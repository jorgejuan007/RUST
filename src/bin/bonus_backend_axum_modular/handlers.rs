use crate::models::{HealthResponse, NewTask, Task, TaskStats};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn salud() -> Json<HealthResponse> {
    Json(HealthResponse {
        servicio: "curso-rust-modular".to_string(),
        estado: "ok".to_string(),
    })
}

pub async fn listar_tareas(State(state): State<AppState>) -> Result<Json<Vec<Task>>, StatusCode> {
    state
        .store
        .list()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn obtener_tarea(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<Task>, StatusCode> {
    match state.store.get(id) {
        Ok(Some(task)) => Ok(Json(task)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn crear_tarea(
    State(state): State<AppState>,
    Json(payload): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>), StatusCode> {
    let titulo = payload.titulo.trim();
    if titulo.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let task = state
        .store
        .add(titulo.to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn obtener_estadisticas(
    State(state): State<AppState>,
) -> Result<Json<TaskStats>, StatusCode> {
    state
        .store
        .stats()
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
