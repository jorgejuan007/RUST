use serde_json::Value;
use std::process::{Command, Output};

fn run(path: &std::path::Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_proyecto_tareas_cli"))
        .arg("--db")
        .arg(path)
        .args(arguments)
        .output()
        .unwrap()
}

fn value(output: Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn cli_y_api_comparten_persistencia() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tasks.sqlite3");
    assert_eq!(value(run(&path, &["add", "Desde CLI"]))["id"], 1);
    assert_eq!(value(run(&path, &["done", "1"]))["hecha"], true);
    let store = rust_30_dias::api::TaskStore::open(&path).unwrap();
    assert!(store.get(1).unwrap().hecha);
    store.add("Desde API").unwrap();
    assert_eq!(value(run(&path, &["list", "--hecha", "false"]))["total"], 1);
    assert_eq!(
        value(run(&path, &["edit", "1", "Renombrada"]))["titulo"],
        "Renombrada"
    );
    assert_eq!(value(run(&path, &["reopen", "1"]))["hecha"], false);
    value(run(&path, &["delete", "1"]));
    assert_eq!(value(run(&path, &["stats"]))["total"], 1);
}

#[test]
fn cli_informa_errores_con_codigo_de_salida() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tasks.sqlite3");
    for args in [
        &["add", " "][..],
        &["get", "99"],
        &["list", "--limit", "0"],
        &["desconocido"],
    ] {
        assert!(!run(&path, args).status.success());
    }
    assert_eq!(value(run(&path, &["stats"]))["total"], 0);
}

#[test]
fn cli_copia_y_recupera_sin_sobrescribir() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.sqlite3");
    let target = directory.path().join("restored.sqlite3");
    let backup = directory.path().join("copia con espacios.backup.json");
    value(run(&source, &["add", "Conservar 🌳"]));
    value(run(&source, &["done", "1"]));
    let path = backup.to_str().unwrap();
    assert_eq!(value(run(&source, &["backup", path]))["tareas"], 1);
    let before = std::fs::read(&backup).unwrap();
    assert!(!run(&source, &["backup", path]).status.success());
    assert_eq!(std::fs::read(&backup).unwrap(), before);
    assert_eq!(value(run(&target, &["restore", path]))["total"], 1);
    assert_eq!(value(run(&target, &["get", "1"]))["hecha"], true);
    assert!(!run(&target, &["restore", path]).status.success());
    assert_eq!(value(run(&target, &["add", "Segunda"]))["id"], 2);
    std::fs::write(&backup, "no es JSON").unwrap();
    assert!(!run(&target, &["restore", path]).status.success());
    assert_eq!(value(run(&target, &["stats"]))["total"], 2);
}
