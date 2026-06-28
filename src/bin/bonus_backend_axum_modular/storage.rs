use crate::models::{Task, TaskStats};
use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct TaskStore {
    inner: Arc<Mutex<Vec<Task>>>,
}

impl TaskStore {
    pub fn seed() -> Self {
        Self {
            inner: Arc::new(Mutex::new(vec![
                Task {
                    id: 1,
                    titulo: "Diseñar handlers".to_string(),
                    hecha: true,
                },
                Task {
                    id: 2,
                    titulo: "Separar storage".to_string(),
                    hecha: false,
                },
            ])),
        }
    }

    pub fn list(&self) -> Result<Vec<Task>> {
        let tasks = self.inner.lock().map_err(|_| anyhow!("mutex envenenado"))?;
        Ok(tasks.clone())
    }

    pub fn get(&self, id: u32) -> Result<Option<Task>> {
        let tasks = self.inner.lock().map_err(|_| anyhow!("mutex envenenado"))?;
        Ok(tasks.iter().find(|task| task.id == id).cloned())
    }

    pub fn add(&self, titulo: String) -> Result<Task> {
        let mut tasks = self.inner.lock().map_err(|_| anyhow!("mutex envenenado"))?;
        let id = tasks.last().map(|task| task.id + 1).unwrap_or(1);

        let task = Task {
            id,
            titulo,
            hecha: false,
        };
        tasks.push(task.clone());
        Ok(task)
    }

    pub fn stats(&self) -> Result<TaskStats> {
        let tasks = self.inner.lock().map_err(|_| anyhow!("mutex envenenado"))?;
        let pendientes = tasks.iter().filter(|task| !task.hecha).count();

        Ok(TaskStats {
            total: tasks.len(),
            pendientes,
        })
    }
}
