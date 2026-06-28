# Backend Local con `axum` y `reqwest`

Esta capa ya te mete en Rust de aplicaciones reales, pero sin depender de Internet ni de servicios externos.

## La idea

En vez de separar teoria de cliente y servidor, aqui conviene ver el circuito completo:

- un servidor HTTP pequeno con `axum`
- un cliente HTTP con `reqwest`
- tipos compartidos serializados con `serde`

Eso deja muy claro como se conectan las piezas.

## `axum` en una frase

`axum` te ayuda a construir APIs HTTP con:

- rutas
- extractores
- estado compartido
- respuestas JSON

## `reqwest` en una frase

`reqwest` es una forma ergonomica de hacer peticiones HTTP desde Rust.

Lo usaras mucho para:

- clientes de API
- integraciones con servicios externos
- pruebas locales de endpoints

## Por que un backend local es buen siguiente paso

Porque te obliga a practicar a la vez:

- `async`
- `Result`
- `serde`
- modelado de tipos
- estado compartido

Y aun asi sigue siendo un proyecto abordable.

## Demo ejecutable del repo

Archivo:

- src/bin/bonus_backend_axum_reqwest.rs (`src/bin/bonus_backend_axum_reqwest.rs`)

Ejecuta:

```bash
cargo run --bin bonus_backend_axum_reqwest
```

Ese bonus:

- levanta un servidor local en un puerto aleatorio
- expone `/salud`
- expone `GET /tasks`
- expone `POST /tasks`
- expone `GET /tasks/{id}`
- usa `reqwest` para llamar a esos endpoints desde el mismo binario

## Que piezas conviene observar

### Tipos serializables

Los mismos structs pueden servir para:

- recibir JSON
- responder JSON
- mantener cierta coherencia entre capas

### Estado compartido

El ejemplo usa `Arc<Mutex<Vec<Task>>>` porque es una forma simple y clara de enseñar:

- datos compartidos
- mutacion coordinada
- acceso desde handlers

No es la arquitectura final ideal para todo, pero es una muy buena base didactica.

### Puerto aleatorio

Se usa `127.0.0.1:0` para que el sistema elija un puerto libre.

Eso evita choques y hace la demo mas robusta.

## Flujo mental sano

1. modela los datos
2. define rutas pequenas
3. separa acceso al estado
4. devuelve JSON claro
5. prueba el endpoint con un cliente real

## Anti patrones comunes

- meter toda la logica en los handlers
- mezclar parseo, validacion y acceso a datos sin orden
- usar strings para todo el payload
- crear un backend async antes de dominar `Result` y `serde`

## Senal de progreso real

Ya estas entrando en backend de verdad cuando puedes:

- definir una ruta con payload tipado
- devolver JSON sin pelearte con el compilador
- entender por que el estado compartido necesita control
- probar un endpoint con un cliente propio

## Siguiente escalon natural

Despues de este bonus, la progresion buena suele ser:

1. meter validaciones de entrada
2. separar estado y handlers en modulos
3. persistir tareas en archivo o base de datos
4. pasar luego a `sqlx` o `rusqlite`

## Cruce recomendado

Este documento encaja especialmente bien con:

- [docs/20_async_practico_con_tokio.md](20_async_practico_con_tokio.md)
- [docs/24_persistencia_con_csv_y_toml.md](24_persistencia_con_csv_y_toml.md)
- [docs/26_backend_axum_modular_y_storage.md](26_backend_axum_modular_y_storage.md)

## Cierre

Backend en Rust deja de parecer abstracto en cuanto haces esta primera vuelta local: ruta, estado, JSON y cliente.
