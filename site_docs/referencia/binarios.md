# Binarios y Comandos

Esta pagina resume como ejecutar el material practico desde la raiz del repositorio.

## Curso diario

```bash
cargo run --bin dia_01_hola_rust
cargo run --bin dia_07_calculadora_cli
cargo run --bin dia_18_operador_q
cargo run --bin dia_30_proyecto_final
```

## Bonus

```bash
cargo run --bin bonus_gestor_tareas_std -- list
cargo run --bin bonus_gestor_tareas_json -- list
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
cargo run --bin bonus_backend_axum_cliente -- health
cargo run --bin proyecto_tareas_cli -- list
```

## Retos completos

```bash
cargo run --bin reto_01_operaciones_extra
cargo run --bin reto_22_modularizado
cargo run --bin reto_30_cli_tareas -- list
```

## Verificaciones

```bash
cargo check --bins
cargo test
cargo test --doc
make ci
mkdocs build --strict
```
