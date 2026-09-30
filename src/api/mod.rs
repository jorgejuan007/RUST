//! API de tareas. La CLI y los handlers comparten el mismo almacenamiento SQLite.

pub mod backup;
mod error;
mod handlers;
pub mod models;
pub mod storage;

use axum::{routing::get, Router};
use handlers::{contrato, crear, editar, eliminar, estadisticas, listar, obtener, salud};
pub use storage::TaskStore;

/// Construye las rutas sin abrir puertos, para poder probarlas como servicios.
pub fn router(store: TaskStore) -> Router {
    Router::new()
        .route("/salud", get(salud))
        .route("/openapi.json", get(contrato))
        .route("/stats", get(estadisticas))
        .route("/tasks", get(listar).post(crear))
        .route("/tasks/{id}", get(obtener).patch(editar).delete(eliminar))
        .fallback(error::ruta_no_encontrada)
        .method_not_allowed_fallback(error::metodo_no_permitido)
        .with_state(store)
}
