# Proyectos Finales y Siguiente Nivel

Terminar los 30 dias no significa "ya lo sabes todo". Significa que ya puedes construir cosas reales sin ir a ciegas.

## Proyecto final recomendado

El proyecto mas natural para cerrar este curso es una CLI de tareas o notas. Intenta que incluya:

- modelo con `struct` y `enum`
- comandos `add`, `list`, `done`, `delete`, `search`
- persistencia en archivo
- errores con `Result`
- tests de la logica central
- separacion en modulos

## Cinco proyectos muy buenos despues del curso

### 1. Gestor de tareas CLI serio

Objetivo:

- dominar `std::env::args`
- organizar modulos
- guardar y cargar datos

Extensiones:

- filtros por estado
- prioridad
- fechas
- exportacion a CSV

### 2. Analizador de texto avanzado

Objetivo:

- practicar `HashMap`
- normalizacion de strings
- iteradores

Extensiones:

- ranking por frecuencia
- eliminacion de stopwords
- conteo por longitud de palabra

### 3. Mini servidor HTTP

Objetivo:

- entrar al backend con Rust
- trabajar con handlers y serializacion

Crates recomendadas:

- `axum`
- `serde`
- `tokio`

### 4. Cliente de API

Objetivo:

- practicar `Result`
- parseo JSON
- trabajo con red

Crates recomendadas:

- `reqwest`
- `serde`
- `serde_json`

### 5. Libreria reutilizable

Objetivo:

- pensar en API publica
- escribir tests limpios
- usar traits y genericos

## Arquitectura minima recomendable

Para proyectos medianos, una buena base suele ser:

```text
src/
|-- main.rs
|-- lib.rs
|-- models.rs
|-- storage.rs
|-- commands.rs
`-- errors.rs
```

## Crates que deberias aprender despues

### `serde`

Para serializar y deserializar structs a JSON, TOML y mas.

### `clap`

Para CLIs profesionales con subcomandos y ayuda automatica.

### `anyhow`

Para manejo de errores ergonomico en aplicaciones.

### `thiserror`

Para definir errores propios de forma limpia.

### `tokio`

Para async real y ecosistema de I/O asincrono.

### `axum`

Para construir APIs y servicios web modernos.

## Rutas segun interes

### Si te interesa backend

Aprende este orden:

1. `serde`
2. `clap`
3. `reqwest`
4. `tokio`
5. `axum`
6. `sqlx`

### Si te interesan CLI y automatizacion

Aprende este orden:

1. `clap`
2. `serde`
3. `csv`
4. `anyhow`
5. `thiserror`

### Si te interesan bibliotecas

Aprende este orden:

1. modules y APIs publicas
2. tests
3. traits y genericos
4. documentacion
5. benchmarks mas adelante

## Que leer despues del curso

- The Rust Programming Language
- Rust by Example
- The Cargo Book
- documentacion de `std`

## Como saber si ya estas listo para ir mas lejos

Estas listo para pasar al siguiente nivel cuando puedes:

- disenar una struct y un enum sin dudar demasiado
- escribir una funcion que devuelva `Result`
- leer errores de ownership sin panico
- separar un proyecto pequeno en modulos
- decidir si un parametro debe ser `String` o `&str`

## Objetivo real del siguiente nivel

No se trata de "usar crates avanzadas". Se trata de que el codigo que escribas sea:

- claro
- idiomatico
- facil de probar
- facil de extender

Ese cambio de mentalidad vale mas que aprender diez crates a medias.
