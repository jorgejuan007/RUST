//! Copias JSON versionadas. La restauración del dominio vive en `TaskStore`.

use super::models::Task;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
use thiserror::Error;

pub const MAX_BACKUP_TASKS: usize = 10_000;
pub const MAX_BACKUP_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Backup {
    pub version: u32,
    /// Último identificador asignado, incluidos los de tareas borradas.
    pub ultimo_id: i64,
    pub tareas: Vec<Task>,
}

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("error de archivo de copia: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON de copia no válido: {0}")]
    Json(#[from] serde_json::Error),
    #[error("la copia supera el límite de 16 MiB")]
    TooLarge,
}

impl Backup {
    /// Limita la lectura incluso si el archivo crece después de abrirlo.
    pub fn read(path: impl AsRef<Path>) -> Result<Self, BackupError> {
        let mut bytes = Vec::new();
        File::open(path)?
            .take(MAX_BACKUP_BYTES + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > MAX_BACKUP_BYTES {
            return Err(BackupError::TooLarge);
        }
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Escribe primero un temporal en la carpeta destino y nunca sobrescribe archivos.
    pub fn write_new(&self, path: impl AsRef<Path>) -> Result<(), BackupError> {
        let bytes = serde_json::to_vec_pretty(self)?;
        if bytes.len() as u64 + 1 > MAX_BACKUP_BYTES {
            return Err(BackupError::TooLarge);
        }
        let path = path.as_ref();
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        fs::create_dir_all(parent)?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
        temporary.write_all(&bytes)?;
        temporary.write_all(b"\n")?;
        temporary.as_file().sync_all()?;
        temporary
            .persist_noclobber(path)
            .map_err(|error| error.error)?;
        Ok(())
    }
}
