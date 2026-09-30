use super::storage::StoreError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    pub fn input(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }
    pub fn internal(error: impl std::fmt::Display) -> Self {
        eprintln!("error interno de la API: {error}");
        Self::input(
            StatusCode::INTERNAL_SERVER_ERROR,
            "error_interno",
            "no se pudo completar la operacion",
        )
    }
}

impl From<StoreError> for ApiError {
    fn from(error: StoreError) -> Self {
        match error {
            StoreError::Invalid(message) => {
                Self::input(StatusCode::BAD_REQUEST, "entrada_invalida", message)
            }
            StoreError::NotFound(id) => Self::input(
                StatusCode::NOT_FOUND,
                "tarea_no_encontrada",
                format!("no existe la tarea {id}"),
            ),
            other => Self::internal(other),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({"error": {"codigo": self.code, "mensaje": self.message}})),
        )
            .into_response()
    }
}

pub async fn ruta_no_encontrada() -> ApiError {
    ApiError::input(
        StatusCode::NOT_FOUND,
        "ruta_no_encontrada",
        "la ruta solicitada no existe",
    )
}
pub async fn metodo_no_permitido() -> ApiError {
    ApiError::input(
        StatusCode::METHOD_NOT_ALLOWED,
        "metodo_no_permitido",
        "metodo no permitido para esta ruta",
    )
}
