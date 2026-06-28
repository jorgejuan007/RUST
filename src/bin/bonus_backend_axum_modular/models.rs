use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: u32,
    pub titulo: String,
    pub hecha: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub titulo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub servicio: String,
    pub estado: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskStats {
    pub total: usize,
    pub pendientes: usize,
}
