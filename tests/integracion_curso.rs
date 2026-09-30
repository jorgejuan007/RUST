use rust_30_dias::numeros::fibonacci;
use rust_30_dias::tareas::TaskManager;
use rust_30_dias::tareas_json::TaskStore;
use rust_30_dias::texto::estadisticas;
use std::env;
use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn fibonacci_desde_api_publica() {
    assert_eq!(fibonacci(7), Ok(vec![0, 1, 1, 2, 3, 5, 8]));
}

#[test]
fn task_manager_desde_integracion() {
    let mut manager = TaskManager::new();
    let id = manager.agregar("Repasar borrowing");

    assert_eq!(id, 1);
    assert_eq!(manager.listar().len(), 1);
    assert!(manager.completar(id));
    assert_eq!(manager.pendientes(), 0);
}

#[test]
fn estadisticas_texto_desde_integracion() {
    let (lineas, palabras, caracteres) = estadisticas("hola rust\naprender compensa");

    assert_eq!(lineas, 2);
    assert_eq!(palabras, 4);
    assert!(caracteres >= 20);
}

#[test]
fn task_store_roundtrip_en_archivo_temporal() -> Result<(), Box<dyn Error>> {
    let mut store = TaskStore::default();
    store.add("Preparar tests de integracion");
    store.add("Comprobar roundtrip JSON");

    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = env::temp_dir().join(format!("rust_30_dias_integracion_{unique}.json"));

    store.save(&path)?;
    let cargado = TaskStore::load(&path)?;

    assert_eq!(cargado.list().len(), 2);
    assert_eq!(cargado.list()[0].titulo, "Preparar tests de integracion");
    assert_eq!(cargado.list()[1].titulo, "Comprobar roundtrip JSON");

    fs::remove_file(path)?;
    Ok(())
}
