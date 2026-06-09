pub mod numeros {
    pub fn factorial(n: u64) -> u64 {
        (1..=n).product::<u64>().max(1)
    }

    pub fn fibonacci(cantidad: usize) -> Vec<u64> {
        let mut resultado = Vec::new();

        match cantidad {
            0 => return resultado,
            1 => {
                resultado.push(0);
                return resultado;
            }
            _ => {
                resultado.push(0);
                resultado.push(1);
            }
        }

        while resultado.len() < cantidad {
            let siguiente = resultado[resultado.len() - 1] + resultado[resultado.len() - 2];
            resultado.push(siguiente);
        }

        resultado
    }

    pub fn maximo<T: PartialOrd + Copy>(a: T, b: T) -> T {
        if a > b {
            a
        } else {
            b
        }
    }
}

pub mod texto {
    use std::collections::HashMap;

    pub fn primera_palabra(texto: &str) -> &str {
        match texto.split_whitespace().next() {
            Some(palabra) => palabra,
            None => "",
        }
    }

    pub fn contar_palabras(texto: &str) -> HashMap<String, usize> {
        let mut conteos = HashMap::new();

        for palabra in texto.split_whitespace() {
            let clave = palabra.to_lowercase();
            let contador = conteos.entry(clave).or_insert(0);
            *contador += 1;
        }

        conteos
    }

    pub fn estadisticas(texto: &str) -> (usize, usize, usize) {
        let lineas = texto.lines().count();
        let palabras = texto.split_whitespace().count();
        let caracteres = texto.chars().count();
        (lineas, palabras, caracteres)
    }
}

pub mod tareas {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Estado {
        Pendiente,
        EnProgreso,
        Hecha,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Task {
        pub id: u32,
        pub titulo: String,
        pub estado: Estado,
    }

    #[derive(Debug, Default)]
    pub struct TaskManager {
        tareas: Vec<Task>,
        siguiente_id: u32,
    }

    impl TaskManager {
        pub fn new() -> Self {
            Self {
                tareas: Vec::new(),
                siguiente_id: 1,
            }
        }

        pub fn agregar(&mut self, titulo: &str) -> u32 {
            let id = self.siguiente_id;
            self.tareas.push(Task {
                id,
                titulo: titulo.to_string(),
                estado: Estado::Pendiente,
            });
            self.siguiente_id += 1;
            id
        }

        pub fn listar(&self) -> &[Task] {
            &self.tareas
        }

        pub fn buscar(&self, id: u32) -> Option<&Task> {
            self.tareas.iter().find(|t| t.id == id)
        }

        pub fn buscar_mut(&mut self, id: u32) -> Option<&mut Task> {
            self.tareas.iter_mut().find(|t| t.id == id)
        }

        pub fn completar(&mut self, id: u32) -> bool {
            match self.buscar_mut(id) {
                Some(tarea) => {
                    tarea.estado = Estado::Hecha;
                    true
                }
                None => false,
            }
        }

        pub fn borrar(&mut self, id: u32) -> bool {
            let longitud_inicial = self.tareas.len();
            self.tareas.retain(|t| t.id != id);
            self.tareas.len() != longitud_inicial
        }

        pub fn pendientes(&self) -> usize {
            self.tareas
                .iter()
                .filter(|t| matches!(t.estado, Estado::Pendiente | Estado::EnProgreso))
                .count()
        }
    }
}

pub mod tareas_json {
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::io;
    use std::path::Path;
    use thiserror::Error;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Estado {
        Pendiente,
        EnProgreso,
        Hecha,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Task {
        pub id: u32,
        pub titulo: String,
        pub estado: Estado,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct TaskStore {
        pub tareas: Vec<Task>,
    }

    #[derive(Debug, Error)]
    pub enum TaskStoreError {
        #[error("error de entrada/salida: {0}")]
        Io(#[from] io::Error),
        #[error("error al parsear JSON: {0}")]
        Json(#[from] serde_json::Error),
    }

    impl TaskStore {
        pub fn load(path: impl AsRef<Path>) -> Result<Self, TaskStoreError> {
            let path = path.as_ref();

            if !path.exists() {
                return Ok(Self::default());
            }

            let contenido = fs::read_to_string(path)?;
            if contenido.trim().is_empty() {
                return Ok(Self::default());
            }

            Ok(serde_json::from_str(&contenido)?)
        }

        pub fn save(&self, path: impl AsRef<Path>) -> Result<(), TaskStoreError> {
            let contenido = serde_json::to_string_pretty(self)?;
            fs::write(path, contenido)?;
            Ok(())
        }

        pub fn siguiente_id(&self) -> u32 {
            self.tareas.iter().map(|t| t.id).max().unwrap_or(0) + 1
        }

        pub fn add(&mut self, titulo: &str) -> u32 {
            let id = self.siguiente_id();
            self.tareas.push(Task {
                id,
                titulo: titulo.to_string(),
                estado: Estado::Pendiente,
            });
            id
        }

        pub fn list(&self) -> &[Task] {
            &self.tareas
        }

        pub fn complete(&mut self, id: u32) -> bool {
            match self.tareas.iter_mut().find(|t| t.id == id) {
                Some(tarea) => {
                    tarea.estado = Estado::Hecha;
                    true
                }
                None => false,
            }
        }

        pub fn delete(&mut self, id: u32) -> bool {
            let len = self.tareas.len();
            self.tareas.retain(|t| t.id != id);
            self.tareas.len() != len
        }

        pub fn search(&self, termino: &str) -> Vec<&Task> {
            let termino = termino.to_lowercase();
            self.tareas
                .iter()
                .filter(|t| t.titulo.to_lowercase().contains(&termino))
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::numeros::*;
    use super::tareas::{Estado, TaskManager};
    use super::tareas_json::{Estado as EstadoJson, TaskStore};
    use super::texto::*;
    use std::env;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn factorial_funciona() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn fibonacci_funciona() {
        assert_eq!(fibonacci(0), Vec::<u64>::new());
        assert_eq!(fibonacci(1), vec![0]);
        assert_eq!(fibonacci(6), vec![0, 1, 1, 2, 3, 5]);
    }

    #[test]
    fn maximo_funciona() {
        assert_eq!(maximo(10, 7), 10);
        assert_eq!(maximo(2.5, 3.5), 3.5);
    }

    #[test]
    fn primera_palabra_funciona() {
        assert_eq!(primera_palabra("rust es veloz"), "rust");
        assert_eq!(primera_palabra(""), "");
    }

    #[test]
    fn contar_palabras_normaliza() {
        let conteos = contar_palabras("Rust rust RUST");
        assert_eq!(conteos.get("rust"), Some(&3));
    }

    #[test]
    fn task_manager_flujo_basico() {
        let mut manager = TaskManager::new();
        let id_1 = manager.agregar("Estudiar");
        let id_2 = manager.agregar("Practicar");

        assert_eq!(id_1, 1);
        assert_eq!(id_2, 2);
        assert_eq!(manager.listar().len(), 2);
        assert_eq!(manager.pendientes(), 2);

        assert!(manager.completar(id_1));
        assert_eq!(
            manager.buscar(id_1).map(|t| &t.estado),
            Some(&Estado::Hecha)
        );
        assert_eq!(manager.pendientes(), 1);

        assert!(manager.borrar(id_2));
        assert_eq!(manager.listar().len(), 1);
    }

    #[test]
    fn task_store_json_flujo_basico() {
        let mut store = TaskStore::default();
        let id = store.add("Estudiar serde");
        assert_eq!(id, 1);
        assert_eq!(store.list().len(), 1);
        assert_eq!(store.search("serde").len(), 1);
        assert!(store.complete(id));
        assert_eq!(store.list()[0].estado, EstadoJson::Hecha);
        assert!(store.delete(id));
        assert!(store.list().is_empty());
    }

    #[test]
    fn task_store_json_persistencia() {
        let mut store = TaskStore::default();
        store.add("Persistir en JSON");

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!("rust_30_dias_{unique}.json"));

        store.save(&path).unwrap();
        let cargado = TaskStore::load(&path).unwrap();
        assert_eq!(cargado.list().len(), 1);
        assert_eq!(cargado.list()[0].titulo, "Persistir en JSON");

        fs::remove_file(path).unwrap();
    }
}
