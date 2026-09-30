use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use rust_30_dias::api::{
    models::{ListOptions, UpdateTask},
    router,
    storage::StoreError,
    TaskStore,
};
use serde_json::{json, Value};
use tower::ServiceExt;

async fn raw(app: &Router, method: &str, uri: &str, body: &str) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_owned()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, value)
}

async fn request(app: &Router, method: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    raw(app, method, uri, &body.to_string()).await
}

fn app() -> Router {
    router(TaskStore::in_memory().unwrap())
}

#[tokio::test]
async fn ciclo_http_completo() {
    let app = app();
    let (status, health) = request(&app, "GET", "/salud", Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(health["estado"], "ok");
    let (status, created) = request(
        &app,
        "POST",
        "/tasks",
        json!({"titulo": "  Aprender SQLite  "}),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["titulo"], "Aprender SQLite");
    assert_eq!(created["hecha"], false);
    let uri = format!("/tasks/{}", created["id"]);
    assert_eq!(
        request(&app, "GET", &uri, Value::Null).await,
        (StatusCode::OK, created)
    );
    let (status, updated) = request(
        &app,
        "PATCH",
        &uri,
        json!({"titulo": "API terminada", "hecha": true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["titulo"], "API terminada");
    assert_eq!(updated["hecha"], true);
    assert_eq!(
        request(&app, "GET", "/stats", Value::Null).await.1,
        json!({"total": 1, "pendientes": 0})
    );
    assert_eq!(
        request(&app, "DELETE", &uri, Value::Null).await,
        (StatusCode::NO_CONTENT, Value::Null)
    );
    assert_eq!(
        request(&app, "GET", &uri, Value::Null).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request(&app, "GET", "/stats", Value::Null).await.1,
        json!({"total": 0, "pendientes": 0})
    );
}

#[tokio::test]
async fn filtros_unicode_y_paginacion() {
    let app = app();
    for title in ["ÁRBOL primero", "árbol segundo", "Otra tarea"] {
        assert_eq!(
            request(&app, "POST", "/tasks", json!({"titulo": title}))
                .await
                .0,
            StatusCode::CREATED
        );
    }
    request(&app, "PATCH", "/tasks/2", json!({"hecha": true})).await;
    let (status, page) = request(
        &app,
        "GET",
        "/tasks?q=%C3%81RBOL&limit=1&offset=1",
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(page["total"], 2);
    assert_eq!(page["tareas"].as_array().unwrap().len(), 1);
    assert_eq!(page["tareas"][0]["id"], 2);
    let (_, pending) = request(&app, "GET", "/tasks?hecha=false", Value::Null).await;
    assert_eq!(pending["total"], 2);
    let (_, done) = request(&app, "GET", "/tasks?hecha=true&q=%C3%A1rbol", Value::Null).await;
    assert_eq!(done["total"], 1);
    assert_eq!(done["tareas"][0]["id"], 2);
    let (_, empty) = request(&app, "GET", "/tasks?offset=99", Value::Null).await;
    assert_eq!(empty["total"], 3);
    assert_eq!(empty["tareas"], json!([]));
}

#[tokio::test]
async fn entradas_invalidas_no_crean_tareas() {
    let app = app();
    for title in ["".to_owned(), " \n\t".to_owned(), "é".repeat(201)] {
        let (status, error) = request(&app, "POST", "/tasks", json!({"titulo": title})).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(error["error"]["codigo"], "entrada_invalida");
    }
    let (status, _) = request(&app, "POST", "/tasks", json!({"titulo": "é".repeat(200)})).await;
    assert_eq!(status, StatusCode::CREATED);
    let (status, _) = request(
        &app,
        "POST",
        "/tasks",
        json!({"titulo": "Bien", "inventado": true}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        request(&app, "GET", "/stats", Value::Null).await.1["total"],
        1
    );
}

#[tokio::test]
async fn errores_de_extractores_son_json() {
    let app = app();
    let (status, body) = raw(&app, "POST", "/tasks", "{").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["codigo"], "json_invalido");
    for path in [
        "/tasks?limit=0",
        "/tasks?limit=101",
        "/tasks?offset=-1",
        "/tasks?hecha=quizas",
        "/tasks?inventado=1",
        "/tasks/no-numero",
        "/tasks/0",
        "/tasks/-1",
    ] {
        let (status, body) = request(&app, "GET", path, Value::Null).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{path}");
        assert!(body["error"]["codigo"].is_string());
    }
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/tasks")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn ediciones_invalidas_son_atomicas() {
    let app = app();
    request(&app, "POST", "/tasks", json!({"titulo": "Original"})).await;
    for patch in [
        json!({}),
        json!({"titulo": " ", "hecha": true}),
        json!({"titulo": "x".repeat(201)}),
    ] {
        assert_eq!(
            request(&app, "PATCH", "/tasks/1", patch).await.0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(
        request(&app, "GET", "/tasks/1", Value::Null).await.1,
        json!({"id": 1, "titulo": "Original", "hecha": false})
    );
}

#[tokio::test]
async fn inexistentes_y_metodos_no_permitidos() {
    let app = app();
    for method in ["GET", "PATCH", "DELETE"] {
        let (status, body) = request(&app, method, "/tasks/99", json!({"hecha": true})).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body["error"]["codigo"], "tarea_no_encontrada");
    }
    assert_eq!(
        request(&app, "GET", "/inexistente", Value::Null).await.1["error"]["codigo"],
        "ruta_no_encontrada"
    );
    let (status, body) = request(&app, "PUT", "/tasks", Value::Null).await;
    assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(body["error"]["codigo"], "metodo_no_permitido");
}

#[tokio::test]
async fn datos_persisten_al_reabrir_la_api() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("subcarpeta/tasks.sqlite3");
    {
        let app = router(TaskStore::open(&path).unwrap());
        request(&app, "POST", "/tasks", json!({"titulo": "Persistir"})).await;
        request(&app, "PATCH", "/tasks/1", json!({"hecha": true})).await;
    }
    let reopened = router(TaskStore::open(&path).unwrap());
    assert_eq!(
        request(&reopened, "GET", "/tasks/1", Value::Null).await.1["hecha"],
        true
    );
    let (_, created) = request(&reopened, "POST", "/tasks", json!({"titulo": "Segunda"})).await;
    assert_eq!(created["id"], 2);
}

#[tokio::test]
async fn creaciones_concurrentes_tienen_ids_unicos() {
    let app = app();
    let mut tasks = tokio::task::JoinSet::new();
    for n in 0..24 {
        let app = app.clone();
        tasks.spawn(async move {
            request(
                &app,
                "POST",
                "/tasks",
                json!({"titulo": format!("Tarea {n}")}),
            )
            .await
        });
    }
    let mut ids = std::collections::HashSet::new();
    while let Some(result) = tasks.join_next().await {
        let (status, body) = result.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert!(ids.insert(body["id"].as_i64().unwrap()));
    }
    assert_eq!(ids.len(), 24);
    assert_eq!(
        request(&app, "GET", "/stats", Value::Null).await.1["total"],
        24
    );
}

#[test]
fn ids_no_se_reutilizan_y_busqueda_es_literal() {
    let store = TaskStore::in_memory().unwrap();
    let first = store.add("100%_SQL").unwrap();
    store.delete(first.id).unwrap();
    let next = store.add("Árbol ' OR 1=1 --").unwrap();
    assert!(next.id > first.id);
    let page = store
        .list(&ListOptions {
            q: Some("%".into()),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(page.total, 0);
    store
        .update(
            next.id,
            &UpdateTask {
                titulo: Some("Título nuevo".into()),
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(
        store
            .list(&ListOptions {
                q: Some("TÍTULO".into()),
                ..Default::default()
            })
            .unwrap()
            .total,
        1
    );
}

#[test]
fn bases_danadas_y_versiones_futuras_no_se_borran() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("damaged.sqlite3");
    std::fs::write(&path, "esto no es SQLite").unwrap();
    assert!(TaskStore::open(&path).is_err());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "esto no es SQLite");
    let path = directory.path().join("future.sqlite3");
    rusqlite::Connection::open(&path)
        .unwrap()
        .pragma_update(None, "user_version", 999)
        .unwrap();
    assert!(matches!(
        TaskStore::open(path),
        Err(StoreError::UnsupportedSchema(999))
    ));
}

#[test]
fn esquema_ajeno_no_se_marca_como_compatible() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("legacy.sqlite3");
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute(
            "CREATE TABLE tasks (id INTEGER PRIMARY KEY, titulo TEXT, estado TEXT)",
            [],
        )
        .unwrap();
    connection
        .execute("INSERT INTO tasks VALUES (1, 'Conservar', 'pendiente')", [])
        .unwrap();
    assert!(matches!(
        TaskStore::open(&path),
        Err(StoreError::IncompatibleSchema)
    ));
    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .unwrap();
    assert_eq!(version, 0);
    let title: String = connection
        .query_row("SELECT titulo FROM tasks WHERE id = 1", [], |row| {
            row.get(0)
        })
        .unwrap();
    assert_eq!(title, "Conservar");
}

#[tokio::test]
async fn cuerpo_demasiado_grande_devuelve_error_json() {
    let app = app();
    let (status, body) = request(
        &app,
        "POST",
        "/tasks",
        json!({"titulo": "x".repeat(3 * 1024 * 1024)}),
    )
    .await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(body["error"]["codigo"], "json_invalido");
}

#[tokio::test]
async fn fallo_sqlite_no_expone_detalles_en_http() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tasks.sqlite3");
    let store = TaskStore::open(&path).unwrap();
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute("DROP TABLE tasks", [])
        .unwrap();
    let (status, body) = request(&router(store), "GET", "/stats", Value::Null).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        body,
        json!({"error": {"codigo": "error_interno", "mensaje": "no se pudo completar la operacion"}})
    );
}
