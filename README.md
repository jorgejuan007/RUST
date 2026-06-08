# Curso de Rust en 30 Dias

Este repositorio ya queda preparado como curso practico completo de Rust. Incluye:

- el manual detallado en [manual_rust_30_dias.md](/Users/legalintermedia/Documents/GitHub/RUST/manual_rust_30_dias.md)
- un proyecto Rust listo para ejecutar
- un ejemplo ejecutable por cada dia en `src/bin/`
- archivos de apoyo en `data/`

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
4. Modificalo para obligarte a entenderlo.
5. Corre `cargo fmt`, `cargo clippy` y `cargo test` cuando aplique.

## Mapa de binarios

- Semana 1: `dia_01_hola_rust` a `dia_07_calculadora_cli`
- Semana 2: `dia_08_string_vs_str` a `dia_14_tareas_memoria`
- Semana 3: `dia_15_enums` a `dia_21_analizador_texto`
- Semana 4: `dia_22_modulos` a `dia_28_libreria_reutilizable`
- Semana 5: `dia_29_concurrencia` y `dia_30_proyecto_final`

## Objetivo del repositorio

La idea no es solo leer teoria. El objetivo es que cierres el curso sabiendo:

- escribir programas pequenos y medianos en Rust
- razonar sobre ownership y borrowing
- modelar dominios con `struct`, `enum`, `Option` y `Result`
- organizar proyectos con `cargo`
- leer errores del compilador sin pelearte con ellos

## Recomendacion final

No avances por velocidad. En Rust, repetir con intencion da mucho mas resultado que cubrir mas temas por dia.
