# Backend modular y almacenamiento compartido

El backend evolucionó de una demostración en memoria a un servidor persistente.
El proceso permanece activo hasta Ctrl+C y conserva las tareas en SQLite.

## Responsabilidades

- [Modelos](https://github.com/jorgejuan007/RUST/blob/main/src/api/models.rs): tareas, cambios y páginas de resultados.
- [Storage](https://github.com/jorgejuan007/RUST/blob/main/src/api/storage.rs): esquema, validación y consultas SQLite.
- [Handlers](https://github.com/jorgejuan007/RUST/blob/main/src/api/handlers.rs): conversión de HTTP a operaciones del storage.
- [Errores](https://github.com/jorgejuan007/RUST/blob/main/src/api/error.rs): respuestas JSON coherentes.
- [Router](https://github.com/jorgejuan007/RUST/blob/main/src/api/mod.rs): rutas componibles y comprobables sin abrir puertos.
- [Servidor](https://github.com/jorgejuan007/RUST/blob/main/src/bin/bonus_backend_axum_modular/main.rs): configuración y cierre ordenado.
- [Cliente](https://github.com/jorgejuan007/RUST/blob/main/src/bin/bonus_backend_axum_cliente.rs): llamadas HTTP independientes.
- [CLI SQLite](https://github.com/jorgejuan007/RUST/blob/main/src/bin/proyecto_tareas_cli.rs): reutiliza el mismo storage.

## Arranque

```bash
cargo run --bin bonus_backend_axum_modular
```

En otra terminal:

```bash
cargo run --bin bonus_backend_axum_cliente -- health
cargo run --bin bonus_backend_axum_cliente -- add "Practicar persistencia"
cargo run --bin bonus_backend_axum_cliente -- list
```

El servidor utiliza `127.0.0.1:3000` y `target/tasks.sqlite3` por defecto. Puedes cambiar
ambos con `--bind` y `--db`. No inserta tareas de ejemplo ni borra la base al arrancar.
Para conservar datos fuera de los artefactos de compilación, elige una ruta propia
con `--db`; `cargo clean` elimina el directorio `target/`.

## Trabajo async y SQLite

El store comparte una conexión mediante `Arc<Mutex<Connection>>`. Los handlers
usan `spawn_blocking` para que las operaciones SQLite síncronas no bloqueen los workers
async de Tokio. Los listados consultan el total y la página dentro de una transacción.
Las consultas usan parámetros; la búsqueda normaliza Unicode a minúsculas en Rust.

Consulta el [proyecto completo y contrato HTTP](28_proyecto_tareas_persistente.md).
