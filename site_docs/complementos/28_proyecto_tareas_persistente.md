# Proyecto final: tareas persistentes con CLI y API

Este proyecto conecta las etapas del curso: dominio en memoria, serialización JSON,
SQLite y HTTP. La CLI SQLite y la API usan el mismo modelo, validación y almacenamiento.

## Recorrido de aprendizaje

1. Estudia `dia_14_tareas_memoria` y el módulo `tareas` de la biblioteca.
2. Ejecuta `bonus_gestor_tareas_json` para aprender serialización y archivos.
3. Usa `proyecto_tareas_cli` para introducir consultas y transacciones SQLite.
4. Arranca `bonus_backend_axum_modular` y usa el cliente HTTP independiente.
5. Lee [las pruebas HTTP](https://github.com/jorgejuan007/RUST/blob/main/tests/api_tareas.rs) y [las pruebas de CLI](https://github.com/jorgejuan007/RUST/blob/main/tests/cli_tareas.rs).

Las etapas iniciales siguen siendo ejemplos independientes. JSON y SQLite usan archivos
distintos; este recorrido no realiza una importación automática de datos anteriores.

## CLI SQLite

```bash
cargo run --bin proyecto_tareas_cli -- add "Terminar el curso"
cargo run --bin proyecto_tareas_cli -- list --hecha false --limit 10
cargo run --bin proyecto_tareas_cli -- get 1
cargo run --bin proyecto_tareas_cli -- edit 1 "Practicar la API"
cargo run --bin proyecto_tareas_cli -- done 1
cargo run --bin proyecto_tareas_cli -- reopen 1
cargo run --bin proyecto_tareas_cli -- stats
cargo run --bin proyecto_tareas_cli -- delete 1
```

Devuelve JSON por salida estándar y un código distinto de cero ante errores.
`--db` se coloca antes del subcomando:

```bash
cargo run --bin proyecto_tareas_cli -- --db ./mis-tareas.sqlite3 add "Estudiar"
```

La API y la CLI deben recibir la misma ruta `--db` para compartir datos. El valor
predeterminado es `target/tasks.sqlite3`; `cargo clean` elimina ese archivo. Usa una
ruta propia para datos que quieras conservar y exclúyela de Git.

## Servidor y cliente

```bash
cargo run --bin bonus_backend_axum_modular -- --bind 127.0.0.1:3000 --db ./mis-tareas.sqlite3
```

El servidor permanece activo y se detiene de forma ordenada con Ctrl+C.
En otra terminal puedes ejecutar:

```bash
cargo run --bin bonus_backend_axum_cliente -- health
cargo run --bin bonus_backend_axum_cliente -- add "Desde HTTP"
cargo run --bin bonus_backend_axum_cliente -- list
```

El cliente acepta `--base http://127.0.0.1:3000` antes del subcomando. También dispone
de `get`, `done`, `delete` y `stats`. Sus peticiones tienen un timeout de 10 segundos.

## Contrato HTTP

| Método | Ruta | Resultado |
|---|---|---|
| GET | `/salud` | Estado del servicio, 200 |
| GET | `/stats` | Total y pendientes, 200 |
| POST | `/tasks` | Crea una tarea, 201 |
| GET | `/tasks` | Página de tareas y total filtrado, 200 |
| GET | `/tasks/{id}` | Una tarea, 200 |
| PATCH | `/tasks/{id}` | Edita título, estado o ambos, 200 |
| DELETE | `/tasks/{id}` | Borra la tarea, 204 sin cuerpo |

Crear:

```bash
curl -sS http://127.0.0.1:3000/tasks -H 'Content-Type: application/json' -d '{"titulo":"Aprender Rust"}'
```

Editar:

```bash
curl -sS -X PATCH http://127.0.0.1:3000/tasks/1 -H 'Content-Type: application/json' -d '{"titulo":"Curso terminado","hecha":true}'
```

Listar y filtrar:

```bash
curl -sS 'http://127.0.0.1:3000/tasks?hecha=false&q=rust&limit=10&offset=0'
```

La página contiene `tareas`, `total`, `limit` y `offset`. El total cuenta todas las
coincidencias antes de paginar. El orden es por identificador ascendente.

- `hecha`: `true` o `false`; omitirlo incluye ambos estados.
- `q`: búsqueda literal, sin distinguir mayúsculas, hasta 200 caracteres.
- `limit`: de 1 a 100; valor predeterminado 20.
- `offset`: entero no negativo; valor predeterminado 0.
- Los títulos se recortan y deben tener de 1 a 200 caracteres Unicode.
- `PATCH` exige al menos `titulo` o `hecha`; una edición inválida no cambia ningún campo.
- Los identificadores son positivos y no se reutilizan después de borrar.

## Errores

Los errores tienen la forma:

```json
{"error":{"codigo":"tarea_no_encontrada","mensaje":"no existe la tarea 99"}}
```

| Estado | Caso |
|---|---|
| 400 | Título, id, consulta o JSON sintáctico inválido |
| 404 | Tarea o ruta inexistente |
| 405 | Método HTTP no permitido |
| 413 | Cuerpo que supera el límite del extractor JSON |
| 415 | Falta `Content-Type: application/json` |
| 422 | JSON con tipos incorrectos, campos desconocidos o datos obligatorios ausentes |
| 500 | Fallo interno de almacenamiento; los detalles se registran en el servidor |

## Persistencia y pruebas

SQLite utiliza un esquema versionado, claves autoincrementales, consultas parametrizadas
y un tiempo de espera para bloqueos de 5 segundos. Las versiones futuras de esquema y
los archivos dañados producen errores; no se borran para ocultar el problema.

```bash
cargo test --test api_tareas
cargo test --test cli_tareas
```

Las pruebas comprueban CRUD, errores HTTP, filtros, Unicode, paginación, concurrencia,
persistencia al reabrir, IDs después de borrar y uso de la misma base desde CLI y API.
Cada prueba usa memoria o una carpeta temporal independiente.

## Límites numéricos de la biblioteca

`numeros::factorial` y `numeros::fibonacci` ahora devuelven `Result`. El factorial de 21
y la serie de 95 elementos no caben en `u64`; devuelven `NumericError::Overflow` tanto
en debug como en release. Propaga el error con `?` o gestiónalo explícitamente.

```rust
use rust_30_dias::numeros::{factorial, NumericError};
assert_eq!(factorial(5), Ok(120));
assert_eq!(factorial(21), Err(NumericError::Overflow));
```

`TaskManager::default()` y `TaskManager::new()` inician ambos los identificadores en 1.
