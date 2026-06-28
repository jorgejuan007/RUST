# Cargo Profesional y Workspaces

Aprender Rust no es solo aprender el lenguaje. Tambien es aprender a trabajar bien con Cargo.

## Comandos que deberias dominar

- `cargo check`
- `cargo run --bin ...`
- `cargo test`
- `cargo test --doc`
- `cargo fmt`
- `cargo doc --no-deps`

Todos ellos ya tienen sentido real en este repositorio.

## Que hace cada uno

- `check`: valida rapido sin enlazar todo como en ejecucion completa
- `test`: compila y ejecuta tests unitarios, integracion y doctests
- `fmt`: normaliza estilo
- `doc`: genera documentacion navegable de tu API

## Artefactos utiles de este repo

- [Cargo.toml](/Users/legalintermedia/Documents/GitHub/RUST/Cargo.toml)
- [Makefile](/Users/legalintermedia/Documents/GitHub/RUST/Makefile)
- [src/lib.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/lib.rs)
- [src/main.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/main.rs)

## `Cargo.toml` como contrato

Ahi declaras:

- nombre del paquete
- edicion
- dependencias
- binario por defecto

No es un detalle administrativo. Es parte del diseno del proyecto.

## Libreria y binarios

Este repo ensena una estructura muy util:

- `src/lib.rs` para logica reusable
- `src/bin/` para programas concretos

Eso es mejor que meter todo en un unico `main.rs`.

## Perfiles `debug` y `release`

Durante estudio y desarrollo:

```bash
cargo run
```

Cuando quieras medir comportamiento mas real:

```bash
cargo run --release --bin bonus_async_tokio_avanzado
```

## `cargo doc`

Ahora que el repo tiene doctests y API publica mas clara, merece la pena generar documentacion Rust nativa:

```bash
cargo doc --no-deps
```

## Makefile local

Ya tienes automatizacion util en:

- [Makefile](/Users/legalintermedia/Documents/GitHub/RUST/Makefile)

La idea correcta es convertir tareas repetibles en comandos cortos.

## Workspaces en una frase

Un workspace agrupa varios crates relacionados bajo una raiz comun.

Conviene cuando tienes:

- varias librerias
- varias apps
- codigo compartido entre paquetes

## Cuando todavia no hace falta

No conviertas un proyecto pequeno en workspace solo por imitar repos grandes.

Primero agota bien:

- `lib.rs`
- `src/bin/`
- modulos internos

## Senales de que un workspace ya tiene sentido

- dos binarios grandes con dependencias muy distintas
- una libreria que quieres reutilizar desde varios paquetes
- ejemplos o herramientas auxiliares que merecen crate propio

## Integracion con CI y docs

Este curso ya conecta Cargo con automatizacion real:

- [docs/17_automatizacion_y_publicacion.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/17_automatizacion_y_publicacion.md)
- [scripts/sync_mkdocs.py](/Users/legalintermedia/Documents/GitHub/RUST/scripts/sync_mkdocs.py)
- [.github/workflows/ci.yml](/Users/legalintermedia/Documents/GitHub/RUST/.github/workflows/ci.yml)

## Flujo recomendable

Antes de subir cambios:

```bash
cargo fmt
cargo test
make ci
```

## Cierre

Dominar Cargo te ahorra una cantidad enorme de friccion. En proyectos reales, esa soltura cuenta tanto como la sintaxis.

## Cruce recomendado

Esta capa encaja especialmente bien con:

- [docs/23_backend_local_con_axum_y_reqwest.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/23_backend_local_con_axum_y_reqwest.md)
- [docs/24_persistencia_con_csv_y_toml.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/24_persistencia_con_csv_y_toml.md)
