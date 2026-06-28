# Curso de Rust en 30 Dias

Este repositorio ya queda preparado como curso practico completo de Rust. Incluye:

- el manual detallado en [manual_rust_30_dias.md](/Users/legalintermedia/Documents/GitHub/RUST/manual_rust_30_dias.md)
- un proyecto Rust listo para ejecutar
- un ejemplo ejecutable por cada dia en `src/bin/`
- archivos de apoyo en `data/`
- documentacion complementaria en [docs/README.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/README.md)
- un proyecto bonus mas completo en `bonus_gestor_tareas_std`
- una libreria reutilizable con modulos y tests en [src/lib.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/lib.rs)
- una capa avanzada con `clap`, `serde`, `serde_json` y `thiserror`
- ejemplos locales de backend con `axum` y `reqwest`
- ejemplos de persistencia con `csv` y `toml`
- una capa siguiente con `rusqlite` y backend modular
- soluciones completas ejecutables para los 30 retos extra
- automatizacion local con `Makefile` y CI/Pages con GitHub Actions

## Como usar el curso

Instala Rust y luego, desde la raiz del repo:

```bash
cargo run
```

Eso te mostrara un indice rapido del curso.

Para ejecutar un dia concreto:

```bash
cargo run --bin dia_01_hola_rust
cargo run --bin dia_07_calculadora_cli
cargo run --bin dia_18_operador_q
cargo run --bin dia_30_proyecto_final
```

Para comprobar que todo compila:

```bash
cargo check --bins
```

## Sitio MkDocs

Para regenerar el contenido web a partir del material del repositorio:

```bash
python3 scripts/sync_mkdocs.py
```

Tambien puedes usar:

```bash
make sync-docs
```

Para levantar el sitio en local:

```bash
mkdocs serve
```

Para construirlo y validar enlaces:

```bash
mkdocs build --strict
```

O bien:

```bash
make docs-build
```

## Automatizacion local

Objetivos disponibles:

```bash
make help
make check
make test
make test-doc
make doc-api
make ci
```

`make ci` regenera `site_docs`, ejecuta `cargo check --bins`, `cargo test` y `mkdocs build --strict`.

Para correr los tests del dia 23:

```bash
cargo test --bin dia_23_tests
```

Para ejecutar bonus:

```bash
cargo run --bin bonus_gestor_tareas_std -- list
cargo run --bin bonus_gestor_tareas_std -- add Practicar ownership
cargo run --bin bonus_gestor_tareas_json -- list
cargo run --bin bonus_gestor_tareas_json -- add Aprender serde
cargo run --bin bonus_uso_libreria
cargo run --bin bonus_option_result_combinadores
cargo run --bin bonus_canales_mutex
cargo run --bin bonus_async_tokio
cargo run --bin bonus_errores_io_anyhow
cargo run --bin bonus_smart_pointers
cargo run --bin bonus_async_tokio_avanzado
cargo run --bin bonus_backend_axum_reqwest
cargo run --bin bonus_persistencia_csv_toml
cargo run --bin bonus_sqlite_rusqlite
cargo run --bin bonus_backend_axum_modular
```

Para ejecutar retos completos:

```bash
cargo run --bin reto_01_operaciones_extra
cargo run --bin reto_22_modularizado
cargo run --bin reto_30_cli_tareas -- list
```

## Estructura

```text
.
|-- Cargo.toml
|-- README.md
|-- data/
|-- docs/
|-- manual_rust_30_dias.md
`-- src/
    |-- lib.rs
    |-- main.rs
    `-- bin/
```

## Ruta sugerida de estudio

1. Lee el dia correspondiente en [manual_rust_30_dias.md](/Users/legalintermedia/Documents/GitHub/RUST/manual_rust_30_dias.md).
2. Ejecuta el binario de ese dia.
3. Reescribe el ejemplo sin mirar.
4. Consulta la documentacion complementaria cuando necesites mas contexto.
5. Resuelve el reto extra correspondiente en `docs/05_retos_extra_30_dias.md`.
6. Modificalo para obligarte a entenderlo.
7. Corre `cargo fmt`, `cargo clippy` y `cargo test` cuando aplique.

## Mapa de binarios

- Semana 1: `dia_01_hola_rust` a `dia_07_calculadora_cli`
- Semana 2: `dia_08_string_vs_str` a `dia_14_tareas_memoria`
- Semana 3: `dia_15_enums` a `dia_21_analizador_texto`
- Semana 4: `dia_22_modulos` a `dia_28_libreria_reutilizable`
- Semana 5: `dia_29_concurrencia` y `dia_30_proyecto_final`
- Bonus: `bonus_gestor_tareas_std`, `bonus_gestor_tareas_json`
- Bonus adicionales: `bonus_uso_libreria`, `bonus_option_result_combinadores`, `bonus_canales_mutex`, `bonus_async_tokio`, `bonus_errores_io_anyhow`, `bonus_smart_pointers`, `bonus_async_tokio_avanzado`, `bonus_backend_axum_reqwest`, `bonus_persistencia_csv_toml`, `bonus_sqlite_rusqlite`, `bonus_backend_axum_modular`
- Retos completos: `reto_01_operaciones_extra` a `reto_30_cli_tareas`

## Material ampliado

- Guia de estudio: [docs/01_guia_de_estudio_y_habitos.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/01_guia_de_estudio_y_habitos.md)
- Modelo mental: [docs/02_modelo_mental_de_rust.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/02_modelo_mental_de_rust.md)
- Chuleta: [docs/03_chuleta_rust_y_cargo.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/03_chuleta_rust_y_cargo.md)
- Diagnostico de errores: [docs/04_diagnostico_de_errores_rust.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/04_diagnostico_de_errores_rust.md)
- Retos extra: [docs/05_retos_extra_30_dias.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/05_retos_extra_30_dias.md)
- Proyectos y siguiente nivel: [docs/06_proyectos_finales_y_siguiente_nivel.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/06_proyectos_finales_y_siguiente_nivel.md)
- Glosario: [docs/07_glosario_rust.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/07_glosario_rust.md)
- Repaso: [docs/08_preguntas_de_repaso_y_respuestas.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/08_preguntas_de_repaso_y_respuestas.md)
- Antipatrones y buenas practicas: [docs/09_antipatrones_y_buenas_practicas.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/09_antipatrones_y_buenas_practicas.md)
- Soluciones de retos: [docs/10_soluciones_a_retos_extra.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/10_soluciones_a_retos_extra.md)
- Evaluacion final: [docs/11_evaluacion_final_y_rubrica.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/11_evaluacion_final_y_rubrica.md)
- Serde, clap y thiserror: [docs/12_serde_clap_thiserror.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/12_serde_clap_thiserror.md)
- Async y Tokio: [docs/13_async_y_tokio.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/13_async_y_tokio.md)
- Arquitectura y refactor: [docs/14_arquitectura_y_refactor_en_rust.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/14_arquitectura_y_refactor_en_rust.md)
- Ruta postcurso: [docs/15_ruta_postcurso.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/15_ruta_postcurso.md)
- Soluciones completas ejecutables: [docs/16_soluciones_completas_ejecutables.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/16_soluciones_completas_ejecutables.md)
- Automatizacion y publicacion: [docs/17_automatizacion_y_publicacion.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/17_automatizacion_y_publicacion.md)
- Errores, I O y anyhow: [docs/18_errores_io_y_anyhow.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/18_errores_io_y_anyhow.md)
- Smart pointers y mutabilidad interior: [docs/19_smart_pointers_y_mutabilidad_interior.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/19_smart_pointers_y_mutabilidad_interior.md)
- Async practico con Tokio: [docs/20_async_practico_con_tokio.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/20_async_practico_con_tokio.md)
- Tests de integracion y doctests: [docs/21_tests_de_integracion_y_doctests.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/21_tests_de_integracion_y_doctests.md)
- Cargo profesional y workspaces: [docs/22_cargo_profesional_y_workspaces.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/22_cargo_profesional_y_workspaces.md)
- Backend local con axum y reqwest: [docs/23_backend_local_con_axum_y_reqwest.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/23_backend_local_con_axum_y_reqwest.md)
- Persistencia con CSV y TOML: [docs/24_persistencia_con_csv_y_toml.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/24_persistencia_con_csv_y_toml.md)
- SQLite con rusqlite: [docs/25_sqlite_con_rusqlite.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/25_sqlite_con_rusqlite.md)
- Backend axum modular y storage: [docs/26_backend_axum_modular_y_storage.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/26_backend_axum_modular_y_storage.md)

## Automatizacion y despliegue

- Makefile local: [Makefile](/Users/legalintermedia/Documents/GitHub/RUST/Makefile)
- Manifiesto del paquete: [Cargo.toml](/Users/legalintermedia/Documents/GitHub/RUST/Cargo.toml)
- Dependencias docs: [requirements-docs.txt](/Users/legalintermedia/Documents/GitHub/RUST/requirements-docs.txt)
- CI: [.github/workflows/ci.yml](/Users/legalintermedia/Documents/GitHub/RUST/.github/workflows/ci.yml)
- GitHub Pages: [.github/workflows/pages.yml](/Users/legalintermedia/Documents/GitHub/RUST/.github/workflows/pages.yml)

## Objetivo del repositorio

La idea no es solo leer teoria. El objetivo es que cierres el curso sabiendo:

- escribir programas pequenos y medianos en Rust
- razonar sobre ownership y borrowing
- modelar dominios con `struct`, `enum`, `Option` y `Result`
- organizar proyectos con `cargo`
- leer errores del compilador sin pelearte con ellos

## Recomendacion final

No avances por velocidad. En Rust, repetir con intencion da mucho mas resultado que cubrir mas temas por dia.
