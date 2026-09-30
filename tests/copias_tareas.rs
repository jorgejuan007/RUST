use rust_30_dias::api::{
    backup::{Backup, BackupError, MAX_BACKUP_BYTES, MAX_BACKUP_TASKS},
    models::{ListOptions, Task, UpdateTask},
    TaskStore,
};

fn example() -> Backup {
    Backup {
        version: 1,
        ultimo_id: 9,
        tareas: vec![Task {
            id: 3,
            titulo: "Árbol 🌳".into(),
            hecha: true,
        }],
    }
}

#[test]
fn copia_recupera_datos_busqueda_y_ids_borrados() {
    let directory = tempfile::tempdir().unwrap();
    let source = TaskStore::open(directory.path().join("source.sqlite3")).unwrap();
    let first = source.add("ÁRBOL 🌳").unwrap();
    source
        .update(
            first.id,
            &UpdateTask {
                hecha: Some(true),
                ..Default::default()
            },
        )
        .unwrap();
    source.add("Pendiente").unwrap();
    let deleted = source.add("Borrada").unwrap();
    source.delete(deleted.id).unwrap();
    let snapshot = source.snapshot().unwrap();
    let path = directory.path().join("copias/tareas.backup.json");
    snapshot.write_new(&path).unwrap();
    let read = Backup::read(&path).unwrap();
    assert_eq!(read, snapshot);
    let target = TaskStore::open(directory.path().join("restored.sqlite3")).unwrap();
    let stats = target.restore(&read).unwrap();
    assert_eq!((stats.total, stats.pendientes), (2, 1));
    assert_eq!(target.snapshot().unwrap(), snapshot);
    assert_eq!(
        target
            .list(&ListOptions {
                q: Some("árbol".into()),
                ..Default::default()
            })
            .unwrap()
            .total,
        1
    );
    assert_eq!(target.add("Nueva").unwrap().id, deleted.id + 1);
}

#[test]
fn copia_vacia_conserva_historial_de_ids() {
    let source = TaskStore::in_memory().unwrap();
    let task = source.add("Temporal").unwrap();
    source.delete(task.id).unwrap();
    let snapshot = source.snapshot().unwrap();
    assert!(snapshot.tareas.is_empty());
    let target = TaskStore::in_memory().unwrap();
    target.restore(&snapshot).unwrap();
    assert_eq!(target.add("Después").unwrap().id, task.id + 1);
}

#[test]
fn restaurar_rechaza_bases_con_datos_o_historial() {
    let target = TaskStore::in_memory().unwrap();
    let task = target.add("Conservar").unwrap();
    let before = target.snapshot().unwrap();
    assert!(target.restore(&example()).is_err());
    assert_eq!(target.snapshot().unwrap(), before);
    target.delete(task.id).unwrap();
    let before = target.snapshot().unwrap();
    assert!(target.restore(&example()).is_err());
    assert_eq!(target.snapshot().unwrap(), before);
}

#[test]
fn copias_invalidas_no_dejan_datos_parciales() {
    let mut variants = Vec::new();
    let mut backup = example();
    backup.version = 2;
    variants.push(backup);
    let mut backup = example();
    backup.ultimo_id = -1;
    variants.push(backup);
    let mut backup = example();
    backup.ultimo_id = 2;
    variants.push(backup);
    let mut backup = example();
    backup.tareas[0].id = 0;
    variants.push(backup);
    let mut backup = example();
    backup.tareas.push(backup.tareas[0].clone());
    variants.push(backup);
    for title in ["", "  Recortar  ", "\0NUL", &"é".repeat(201)] {
        let mut backup = example();
        backup.tareas[0].titulo = title.into();
        variants.push(backup);
    }
    for backup in variants {
        let target = TaskStore::in_memory().unwrap();
        assert!(target.restore(&backup).is_err(), "{backup:?}");
        assert_eq!(target.snapshot().unwrap().ultimo_id, 0);
        assert_eq!(target.stats().unwrap().total, 0);
        assert_eq!(target.add("Primera").unwrap().id, 1);
    }
}

#[test]
fn error_durante_insercion_revierte_toda_la_restauracion() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tasks.sqlite3");
    let target = TaskStore::open(&path).unwrap();
    rusqlite::Connection::open(&path).unwrap().execute_batch(
        "CREATE TRIGGER simular_fallo BEFORE INSERT ON tasks WHEN NEW.id = 4 BEGIN SELECT RAISE(ABORT, 'fallo simulado'); END;"
    ).unwrap();
    let mut backup = example();
    backup.tareas.push(Task {
        id: 4,
        titulo: "Segunda".into(),
        hecha: false,
    });
    assert!(target.restore(&backup).is_err());
    assert_eq!(target.stats().unwrap().total, 0);
    assert_eq!(target.snapshot().unwrap().ultimo_id, 0);
    assert_eq!(target.add("Primera").unwrap().id, 1);
}

#[test]
fn archivo_existente_y_json_desconocido_se_rechazan() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tareas.backup.json");
    std::fs::write(&path, b"Conservar archivo existente").unwrap();
    assert!(example().write_new(&path).is_err());
    assert_eq!(
        std::fs::read(&path).unwrap(),
        b"Conservar archivo existente"
    );
    for invalid in [
        r#"{"version":1,"ultimo_id":0,"tareas":[],"sorpresa":true}"#,
        r#"{"version":1,"ultimo_id":1,"tareas":[{"id":1,"titulo":"X","hecha":false,"extra":1}]}"#,
        r#"{"version":1,"ultimo_id":0,"tareas":[]} basura"#,
        r#"{"version":1,"ultimo_id":0,"tareas":[]} {}"#,
    ] {
        std::fs::write(&path, invalid).unwrap();
        assert!(matches!(Backup::read(&path), Err(BackupError::Json(_))));
    }
}

#[test]
fn limites_de_archivo_y_numero_de_tareas() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("large.json");
    std::fs::File::create(&path)
        .unwrap()
        .set_len(MAX_BACKUP_BYTES + 1)
        .unwrap();
    assert!(matches!(Backup::read(&path), Err(BackupError::TooLarge)));
    let mut backup = example();
    backup.ultimo_id = MAX_BACKUP_TASKS as i64 + 1;
    backup.tareas = (1..=backup.ultimo_id)
        .map(|id| Task {
            id,
            titulo: "Tarea".into(),
            hecha: false,
        })
        .collect();
    assert!(TaskStore::in_memory().unwrap().restore(&backup).is_err());
}

#[test]
fn restauraciones_concurrentes_no_se_mezclan() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tasks.sqlite3");
    let stores = [
        TaskStore::open(&path).unwrap(),
        TaskStore::open(&path).unwrap(),
    ];
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let threads = stores
        .into_iter()
        .map(|store| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                store.restore(&example())
            })
        })
        .collect::<Vec<_>>();
    let successes = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .filter(Result::is_ok)
        .count();
    assert_eq!(successes, 1);
    let target = TaskStore::open(&path).unwrap();
    assert_eq!(target.snapshot().unwrap(), example());
}
