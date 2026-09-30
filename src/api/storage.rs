use super::{
    backup::{Backup, MAX_BACKUP_TASKS},
    models::{ListOptions, Task, TaskPage, TaskStats, UpdateTask},
};
use rusqlite::{params, Connection, OptionalExtension, Row, TransactionBehavior};
use std::{
    collections::HashSet,
    fs,
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
    time::Duration,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("error SQLite: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("error de archivo: {0}")]
    Io(#[from] std::io::Error),
    #[error("almacenamiento bloqueado por un fallo anterior")]
    Poisoned,
    #[error("no existe la tarea {0}")]
    NotFound(i64),
    #[error("{0}")]
    Invalid(String),
    #[error("version de esquema no compatible: {0}")]
    UnsupportedSchema(i64),
    #[error("el archivo no contiene el esquema de tareas de este proyecto")]
    IncompatibleSchema,
}

#[derive(Clone)]
pub struct TaskStore {
    connection: Arc<Mutex<Connection>>,
}

fn task_from_row(row: &Row<'_>) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        titulo: row.get(1)?,
        hecha: row.get(2)?,
    })
}

fn validate_id(id: i64) -> Result<(), StoreError> {
    if id <= 0 {
        return Err(StoreError::Invalid("el id debe ser positivo".into()));
    }
    Ok(())
}

fn validate_title(title: &str) -> Result<String, StoreError> {
    let title = title.trim();
    if title.contains('\0') {
        return Err(StoreError::Invalid(
            "el título no admite caracteres NUL".into(),
        ));
    }
    if title.is_empty() || title.chars().count() > 200 {
        return Err(StoreError::Invalid(
            "el titulo debe tener entre 1 y 200 caracteres".into(),
        ));
    }
    Ok(title.to_owned())
}

impl TaskStore {
    /// Abre una base persistente sin borrar ni insertar datos de demostracion.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        Self::initialize(Connection::open(path)?)
    }

    pub fn in_memory() -> Result<Self, StoreError> {
        Self::initialize(Connection::open_in_memory()?)
    }

    fn initialize(mut connection: Connection) -> Result<Self, StoreError> {
        connection.busy_timeout(Duration::from_secs(5))?;
        let version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
        if !matches!(version, 0 | 1) {
            return Err(StoreError::UnsupportedSchema(version));
        }
        let columns = {
            let mut statement = connection.prepare("PRAGMA table_info(tasks)")?;
            let rows = statement.query_map([], |row| row.get::<_, String>(1))?;
            rows.collect::<Result<Vec<_>, _>>()?
        };
        if (version == 1 || !columns.is_empty())
            && !["id", "titulo", "busqueda", "hecha"]
                .iter()
                .all(|name| columns.iter().any(|column| column == name))
        {
            return Err(StoreError::IncompatibleSchema);
        }
        connection.pragma_update(None, "journal_mode", "WAL")?;
        match version {
            0 => {
                let transaction = connection.transaction()?;
                transaction.execute_batch(
                    "CREATE TABLE IF NOT EXISTS tasks (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        titulo TEXT NOT NULL CHECK(length(titulo) BETWEEN 1 AND 200),
                        busqueda TEXT NOT NULL,
                        hecha INTEGER NOT NULL DEFAULT 0 CHECK(hecha IN (0, 1))
                    ); PRAGMA user_version = 1;",
                )?;
                transaction.commit()?;
            }
            1 => {}
            _ => unreachable!("la version se valido antes de modificar el esquema"),
        }
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    fn connection(&self) -> Result<MutexGuard<'_, Connection>, StoreError> {
        self.connection.lock().map_err(|_| StoreError::Poisoned)
    }

    pub fn add(&self, title: &str) -> Result<Task, StoreError> {
        let title = validate_title(title)?;
        Ok(self.connection()?.query_row(
            "INSERT INTO tasks (titulo, busqueda) VALUES (?1, ?2) RETURNING id, titulo, hecha",
            params![title, title.to_lowercase()],
            task_from_row,
        )?)
    }

    pub fn get(&self, id: i64) -> Result<Task, StoreError> {
        validate_id(id)?;
        self.connection()?
            .query_row(
                "SELECT id, titulo, hecha FROM tasks WHERE id = ?1",
                [id],
                task_from_row,
            )
            .optional()?
            .ok_or(StoreError::NotFound(id))
    }

    pub fn list(&self, options: &ListOptions) -> Result<TaskPage, StoreError> {
        let limit = options.limit.unwrap_or(20);
        let offset = options.offset.unwrap_or(0);
        if !(1..=100).contains(&limit) {
            return Err(StoreError::Invalid("limit debe estar entre 1 y 100".into()));
        }
        let search = options.q.as_ref().map(|q| q.trim().to_lowercase());
        if search.as_ref().is_some_and(|q| q.chars().count() > 200) {
            return Err(StoreError::Invalid(
                "q admite como maximo 200 caracteres".into(),
            ));
        }
        let mut connection = self.connection()?;
        let transaction = connection.transaction()?;
        let total: i64 = transaction.query_row(
            "SELECT COUNT(*) FROM tasks WHERE (?1 IS NULL OR hecha = ?1) AND (?2 IS NULL OR instr(busqueda, ?2) > 0)",
            params![options.hecha, search], |row| row.get(0),
        )?;
        let tareas = {
            let mut statement = transaction.prepare(
                "SELECT id, titulo, hecha FROM tasks WHERE (?1 IS NULL OR hecha = ?1)
                 AND (?2 IS NULL OR instr(busqueda, ?2) > 0) ORDER BY id LIMIT ?3 OFFSET ?4",
            )?;
            let rows = statement
                .query_map(params![options.hecha, search, limit, offset], task_from_row)?;
            rows.collect::<Result<Vec<_>, _>>()?
        };
        transaction.commit()?;
        Ok(TaskPage {
            tareas,
            total: total as u64,
            limit,
            offset,
        })
    }

    pub fn update(&self, id: i64, changes: &UpdateTask) -> Result<Task, StoreError> {
        validate_id(id)?;
        if changes.titulo.is_none() && changes.hecha.is_none() {
            return Err(StoreError::Invalid(
                "indica titulo o hecha para editar".into(),
            ));
        }
        let title = changes.titulo.as_deref().map(validate_title).transpose()?;
        let search = title.as_ref().map(|title| title.to_lowercase());
        self.connection()?
            .query_row(
                "UPDATE tasks SET titulo = COALESCE(?1, titulo), busqueda = COALESCE(?2, busqueda),
             hecha = COALESCE(?3, hecha) WHERE id = ?4 RETURNING id, titulo, hecha",
                params![title, search, changes.hecha, id],
                task_from_row,
            )
            .optional()?
            .ok_or(StoreError::NotFound(id))
    }

    pub fn delete(&self, id: i64) -> Result<(), StoreError> {
        validate_id(id)?;
        if self
            .connection()?
            .execute("DELETE FROM tasks WHERE id = ?1", [id])?
            == 0
        {
            return Err(StoreError::NotFound(id));
        }
        Ok(())
    }

    pub fn stats(&self) -> Result<TaskStats, StoreError> {
        let (total, pending): (i64, i64) = self.connection()?.query_row(
            "SELECT COUNT(*), COALESCE(SUM(CASE WHEN hecha = 0 THEN 1 ELSE 0 END), 0) FROM tasks",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        Ok(TaskStats {
            total: total as u64,
            pendientes: pending as u64,
        })
    }

    /// Captura todas las tareas y la secuencia de IDs en la misma transacción.
    pub fn snapshot(&self) -> Result<Backup, StoreError> {
        let mut connection = self.connection()?;
        let transaction = connection.transaction()?;
        let count: i64 =
            transaction.query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))?;
        if count > MAX_BACKUP_TASKS as i64 {
            return Err(StoreError::Invalid(
                "las copias admiten hasta 10000 tareas".into(),
            ));
        }
        let ultimo_id = transaction
            .query_row(
                "SELECT seq FROM sqlite_sequence WHERE name = 'tasks'",
                [],
                |row| row.get(0),
            )
            .optional()?
            .unwrap_or(0);
        let tareas = {
            let mut statement =
                transaction.prepare("SELECT id, titulo, hecha FROM tasks ORDER BY id")?;
            let rows = statement.query_map([], task_from_row)?;
            rows.collect::<Result<Vec<_>, _>>()?
        };
        transaction.commit()?;
        Ok(Backup {
            version: 1,
            ultimo_id,
            tareas,
        })
    }

    /// Restaura en una base sin tareas ni historial de IDs. Nunca borra datos existentes.
    pub fn restore(&self, backup: &Backup) -> Result<TaskStats, StoreError> {
        if backup.version != 1 {
            return Err(StoreError::Invalid(format!(
                "versión de copia no compatible: {}",
                backup.version
            )));
        }
        if backup.tareas.len() > MAX_BACKUP_TASKS || backup.ultimo_id < 0 {
            return Err(StoreError::Invalid(
                "copia con tamaño o secuencia de IDs no válidos".into(),
            ));
        }
        let mut ids = HashSet::new();
        for task in &backup.tareas {
            validate_id(task.id)?;
            if task.id > backup.ultimo_id || !ids.insert(task.id) {
                return Err(StoreError::Invalid(
                    "copia con IDs duplicados o superiores a ultimo_id".into(),
                ));
            }
            if validate_title(&task.titulo)? != task.titulo {
                return Err(StoreError::Invalid(
                    "los títulos de la copia deben estar recortados".into(),
                ));
            }
        }
        let mut connection = self.connection()?;
        // Reservar la escritura evita que otra conexión inserte entre el chequeo y la restauración.
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let count: i64 =
            transaction.query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))?;
        let sequence: i64 = transaction
            .query_row(
                "SELECT seq FROM sqlite_sequence WHERE name = 'tasks'",
                [],
                |row| row.get(0),
            )
            .optional()?
            .unwrap_or(0);
        if count != 0 || sequence != 0 {
            return Err(StoreError::Invalid(
                "restaura en una base nueva, sin tareas ni historial de IDs".into(),
            ));
        }
        {
            let mut statement = transaction.prepare(
                "INSERT INTO tasks (id, titulo, busqueda, hecha) VALUES (?1, ?2, ?3, ?4)",
            )?;
            for task in &backup.tareas {
                statement.execute(params![
                    task.id,
                    task.titulo,
                    task.titulo.to_lowercase(),
                    task.hecha
                ])?;
            }
        }
        transaction.execute("DELETE FROM sqlite_sequence WHERE name = 'tasks'", [])?;
        transaction.execute(
            "INSERT INTO sqlite_sequence (name, seq) VALUES ('tasks', ?1)",
            [backup.ultimo_id],
        )?;
        transaction.commit()?;
        Ok(TaskStats {
            total: backup.tareas.len() as u64,
            pendientes: backup.tareas.iter().filter(|task| !task.hecha).count() as u64,
        })
    }
}
