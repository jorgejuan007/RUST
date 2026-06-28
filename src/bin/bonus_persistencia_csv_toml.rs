use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TaskRecord {
    id: u32,
    titulo: String,
    estado: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct AppConfig {
    curso: CursoConfig,
    export: ExportConfig,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct CursoConfig {
    autor: String,
    modo: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct ExportConfig {
    incluir_hechas: bool,
    limite: usize,
    archivo_salida: String,
}

fn parse_tasks_from_reader<R: Read>(reader: R) -> Result<Vec<TaskRecord>> {
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut tasks = Vec::new();

    for row in csv_reader.deserialize() {
        let task: TaskRecord = row.context("fila CSV invalida")?;
        tasks.push(task);
    }

    Ok(tasks)
}

fn load_tasks(path: &Path) -> Result<Vec<TaskRecord>> {
    let file = fs::File::open(path)
        .with_context(|| format!("no se pudo abrir el CSV {}", path.display()))?;
    parse_tasks_from_reader(file)
        .with_context(|| format!("no se pudo interpretar el CSV {}", path.display()))
}

fn load_config(path: &Path) -> Result<AppConfig> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("no se pudo leer el TOML {}", path.display()))?;
    toml::from_str(&content).with_context(|| format!("TOML invalido en {}", path.display()))
}

fn filtrar_exportables(tasks: &[TaskRecord], config: &AppConfig) -> Vec<TaskRecord> {
    tasks
        .iter()
        .filter(|task| config.export.incluir_hechas || task.estado != "hecha")
        .take(config.export.limite)
        .cloned()
        .collect()
}

fn save_export(path: &Path, tasks: &[TaskRecord]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "no se pudo crear el directorio de salida {}",
                parent.display()
            )
        })?;
    }

    let mut writer = csv::Writer::from_path(path)
        .with_context(|| format!("no se pudo abrir el export CSV {}", path.display()))?;

    for task in tasks {
        writer
            .serialize(task)
            .with_context(|| format!("no se pudo escribir la tarea {}", task.id))?;
    }

    writer
        .flush()
        .with_context(|| format!("no se pudo cerrar el CSV {}", path.display()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let csv_path = Path::new("data/bonus_tareas.csv");
    let toml_path = Path::new("data/bonus_config.toml");

    let config = load_config(toml_path)?;
    let tasks = load_tasks(csv_path)?;
    let exportables = filtrar_exportables(&tasks, &config);
    let output_path = Path::new(&config.export.archivo_salida);

    save_export(output_path, &exportables)?;

    println!("Autor del curso: {}", config.curso.autor);
    println!("Modo de ejemplo: {}", config.curso.modo);
    println!("Tareas leidas desde CSV: {}", tasks.len());
    println!("Tareas exportadas: {}", exportables.len());
    println!("Archivo de salida: {}", output_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{filtrar_exportables, parse_tasks_from_reader, AppConfig};

    #[test]
    fn parsea_tareas_desde_csv() {
        let data = "id,titulo,estado\n1,Repasar ownership,pendiente\n2,Practicar tests,hecha\n";

        let tasks = parse_tasks_from_reader(data.as_bytes()).unwrap();

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].titulo, "Repasar ownership");
        assert_eq!(tasks[1].estado, "hecha");
    }

    #[test]
    fn parsea_config_desde_toml() {
        let raw = r#"
[curso]
autor = "Legal Intermedia"
modo = "persistencia-local"

[export]
incluir_hechas = false
limite = 2
archivo_salida = "target/test.csv"
"#;

        let config: AppConfig = toml::from_str(raw).unwrap();

        assert_eq!(config.curso.autor, "Legal Intermedia");
        assert_eq!(config.export.limite, 2);
        assert!(!config.export.incluir_hechas);
    }

    #[test]
    fn filtra_tareas_para_exportar() {
        let data = "id,titulo,estado\n1,Repasar ownership,pendiente\n2,Practicar tests,hecha\n3,Montar CLI,en_progreso\n";
        let tasks = parse_tasks_from_reader(data.as_bytes()).unwrap();
        let raw = r#"
[curso]
autor = "Legal Intermedia"
modo = "persistencia-local"

[export]
incluir_hechas = false
limite = 5
archivo_salida = "target/test.csv"
"#;
        let config: AppConfig = toml::from_str(raw).unwrap();

        let exportables = filtrar_exportables(&tasks, &config);

        assert_eq!(exportables.len(), 2);
        assert_eq!(exportables[0].id, 1);
        assert_eq!(exportables[1].id, 3);
    }
}
