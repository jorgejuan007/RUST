# Backend `axum` Modular y Storage

La primera demo de backend local mostraba el circuito completo. Esta segunda capa enseña como empezar a organizarlo mejor.

## Objetivo real

No se trata de hacer "mas archivos" por decorar.

Se trata de separar responsabilidades:

- modelos
- handlers
- estado compartido
- storage
- arranque del servidor

## Demo ejecutable del repo

Carpeta:

- src/bin/bonus_backend_axum_modular/main.rs (`src/bin/bonus_backend_axum_modular/main.rs`)

Archivos asociados:

- models.rs (`src/bin/bonus_backend_axum_modular/models.rs`)
- handlers.rs (`src/bin/bonus_backend_axum_modular/handlers.rs`)
- state.rs (`src/bin/bonus_backend_axum_modular/state.rs`)
- storage.rs (`src/bin/bonus_backend_axum_modular/storage.rs`)

Ejecuta:

```bash
cargo run --bin bonus_backend_axum_modular
```

Ese bonus:

- levanta una API local
- separa handlers del storage
- expone `/salud`, `/tasks`, `/tasks/{id}` y `/stats`
- valida titulos vacios con `400 BAD REQUEST`
- prueba su propio flujo con `reqwest`

## Por que esta estructura escala mejor

### `models.rs`

Agrupa los tipos del dominio y del transporte.

### `storage.rs`

Centraliza operaciones de lectura, insercion y estadisticas.

### `state.rs`

Declara de forma clara que parte del estado comparten los handlers.

### `handlers.rs`

Convierte HTTP en llamadas a tu logica.

### `main.rs`

Se queda sobre todo con:

- composicion
- rutas
- arranque del servidor
- demo cliente

## Regla practica muy util

Cuando una API empieza a crecer, `main.rs` deberia adelgazar y `storage` deberia absorber cada vez mas logica reutilizable.

## Anti patrones comunes

- handlers con demasiada logica de negocio
- estado compartido definido de forma implicita
- validacion mezclada con serializacion y acceso a datos
- storage sin una API clara

## Siguiente paso natural

La evolucion mas coherente desde aqui es sustituir el storage en memoria por SQLite.

No hace falta hacerlo en el mismo salto, pero el camino ya queda preparado.

## Cruce recomendado

Este documento enlaza muy bien con:

- [docs/23_backend_local_con_axum_y_reqwest.md](23_backend_local_con_axum_y_reqwest.md)
- [docs/25_sqlite_con_rusqlite.md](25_sqlite_con_rusqlite.md)
- [docs/14_arquitectura_y_refactor_en_rust.md](14_arquitectura_y_refactor_en_rust.md)

## Cierre

Separar backend en modulos no es burocracia. Es lo que te permite seguir creciendo sin convertir cada handler en un bloque inmanejable.
