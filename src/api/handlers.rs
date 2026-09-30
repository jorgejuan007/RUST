use super::{
    error::ApiError,
    models::{ListOptions, NewTask, Task, TaskPage, TaskStats, UpdateTask},
    storage::{StoreError, TaskStore},
};
use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        Path, Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};

// SQLite es sincronico: las consultas nunca se ejecutan en los workers async.
async fn execute<T, F>(store: TaskStore, operation: F) -> Result<T, ApiError>
where
    T: Send + 'static,
    F: FnOnce(TaskStore) -> Result<T, StoreError> + Send + 'static,
{
    tokio::task::spawn_blocking(move || operation(store))
        .await
        .map_err(ApiError::internal)?
        .map_err(ApiError::from)
}

fn id_from_path(path: Result<Path<i64>, PathRejection>) -> Result<i64, ApiError> {
    path.map(|Path(id)| id)
        .map_err(|error| ApiError::input(error.status(), "id_invalido", error.body_text()))
}
fn json_error(error: JsonRejection) -> ApiError {
    ApiError::input(error.status(), "json_invalido", error.body_text())
}

pub async fn salud() -> Json<Value> {
    Json(json!({"servicio": "curso-rust-modular", "estado": "ok"}))
}

pub async fn contrato() -> impl IntoResponse {
    (
        [("content-type", "application/json; charset=utf-8")],
        include_str!("openapi.json"),
    )
}

pub async fn listar(
    State(store): State<TaskStore>,
    query: Result<Query<ListOptions>, QueryRejection>,
) -> Result<Json<TaskPage>, ApiError> {
    let Query(options) = query.map_err(|error| {
        ApiError::input(
            StatusCode::BAD_REQUEST,
            "consulta_invalida",
            error.body_text(),
        )
    })?;
    execute(store, move |store| store.list(&options))
        .await
        .map(Json)
}
pub async fn obtener(
    State(store): State<TaskStore>,
    path: Result<Path<i64>, PathRejection>,
) -> Result<Json<Task>, ApiError> {
    let id = id_from_path(path)?;
    execute(store, move |store| store.get(id)).await.map(Json)
}
pub async fn crear(
    State(store): State<TaskStore>,
    payload: Result<Json<NewTask>, JsonRejection>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    let Json(payload) = payload.map_err(json_error)?;
    let task = execute(store, move |store| store.add(&payload.titulo)).await?;
    Ok((StatusCode::CREATED, Json(task)))
}
pub async fn editar(
    State(store): State<TaskStore>,
    path: Result<Path<i64>, PathRejection>,
    payload: Result<Json<UpdateTask>, JsonRejection>,
) -> Result<Json<Task>, ApiError> {
    let id = id_from_path(path)?;
    let Json(payload) = payload.map_err(json_error)?;
    execute(store, move |store| store.update(id, &payload))
        .await
        .map(Json)
}
pub async fn eliminar(
    State(store): State<TaskStore>,
    path: Result<Path<i64>, PathRejection>,
) -> Result<StatusCode, ApiError> {
    let id = id_from_path(path)?;
    execute(store, move |store| store.delete(id)).await?;
    Ok(StatusCode::NO_CONTENT)
}
pub async fn estadisticas(State(store): State<TaskStore>) -> Result<Json<TaskStats>, ApiError> {
    execute(store, |store| store.stats()).await.map(Json)
}
