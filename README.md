# Curso de Rust en 30 Dias

Este repositorio ya queda preparado como curso practico completo de Rust. Incluye:

- el manual detallado en [manual_rust_30_dias.md](/Users/legalintermedia/Documents/GitHub/RUST/manual_rust_30_dias.md)
- un proyecto Rust listo para ejecutar
- un ejemplo ejecutable por cada dia en `src/bin/`
- archivos de apoyo en `data/`
- documentacion complementaria en [docs/README.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/README.md)
- un proyecto bonus mas completo en `bonus_gestor_tareas_std`

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

Para correr los tests del dia 23:

```bash
cargo test --bin dia_23_tests
```

Para ejecutar el proyecto bonus:

```bash
cargo run --bin bonus_gestor_tareas_std -- list
cargo run --bin bonus_gestor_tareas_std -- add Practicar ownership
```

## Estructura

```text
.
├── Cargo.toml
├── README.md
├── data/
├── manual_rust_30_dias.md
└── src/
    ├── main.rs
    └── bin/
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
- Bonus: `bonus_gestor_tareas_std`

## Material ampliado

- Guia de estudio: [docs/01_guia_de_estudio_y_habitos.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/01_guia_de_estudio_y_habitos.md)
- Modelo mental: [docs/02_modelo_mental_de_rust.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/02_modelo_mental_de_rust.md)
- Chuleta: [docs/03_chuleta_rust_y_cargo.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/03_chuleta_rust_y_cargo.md)
- Diagnostico de errores: [docs/04_diagnostico_de_errores_rust.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/04_diagnostico_de_errores_rust.md)
- Retos extra: [docs/05_retos_extra_30_dias.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/05_retos_extra_30_dias.md)
- Proyectos y siguiente nivel: [docs/06_proyectos_finales_y_siguiente_nivel.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/06_proyectos_finales_y_siguiente_nivel.md)

## Objetivo del repositorio

La idea no es solo leer teoria. El objetivo es que cierres el curso sabiendo:

- escribir programas pequenos y medianos en Rust
- razonar sobre ownership y borrowing
- modelar dominios con `struct`, `enum`, `Option` y `Result`
- organizar proyectos con `cargo`
- leer errores del compilador sin pelearte con ellos

## Recomendacion final

No avances por velocidad. En Rust, repetir con intencion da mucho mas resultado que cubrir mas temas por dia.
