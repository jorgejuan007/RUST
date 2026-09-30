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
