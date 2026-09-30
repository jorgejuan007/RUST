# Persistencia con CSV y TOML

No toda persistencia necesita una base de datos desde el primer dia.

Para muchas herramientas pequenas, dos formatos muy utiles son:

- CSV para datos tabulares
- TOML para configuracion

## Cuando usar CSV

CSV encaja bien cuando tienes:

- filas simples
- importacion o exportacion facil
- datos faciles de abrir tambien desde otras herramientas

Ejemplos:

- tareas
- usuarios sencillos
- reportes
- inventarios pequenos

## Cuando usar TOML

TOML encaja muy bien para configuracion:

- opciones legibles
- secciones claras
- valores tipados

En Rust es una opcion muy natural para configurar CLIs o servicios pequenos.

## Por que esta combinacion funciona tan bien

Porque separa dos responsabilidades:

- CSV guarda datos operativos
- TOML decide como tratarlos

Ese corte suele ser mas limpio que meterlo todo en un solo archivo.

## Demo ejecutable del repo

Archivos:

- [src/bin/bonus_persistencia_csv_toml.rs](../src/bin/bonus_persistencia_csv_toml.rs)
- [data/bonus_tareas.csv](../data/bonus_tareas.csv)
- [data/bonus_config.toml](../data/bonus_config.toml)

Ejecuta:

```bash
cargo run --bin bonus_persistencia_csv_toml
```

Ese bonus:

- carga tareas desde CSV
- carga configuracion desde TOML
- filtra que tareas exportar
- escribe un nuevo CSV de salida en `target/bonus_export_tareas.csv`

## Que merece la pena observar

### Parseo desacoplado

El ejemplo separa:

- parseo desde `Read`
- carga desde ruta real

Eso hace que los tests sean mucho mas faciles.

### `serde` en ambos frentes

Con `serde` puedes:

- deserializar CSV a structs
- deserializar TOML a structs
- volver a serializar para exportar

La gracia esta en que el modelo de datos sigue siendo tuyo, no del formato.

### Export controlado por config

El TOML decide:

- si incluir tareas hechas
- cuantas filas exportar
- donde guardar el archivo resultante

Eso ya es una forma muy realista de trabajar.

## Cuando subir a SQLite

CSV y TOML dejan de bastar cuando necesitas:

- muchas actualizaciones
- consultas mas ricas
- relaciones entre entidades
- concurrencia seria

En ese punto ya compensa mirar SQLite.

## Anti patrones comunes

- leer archivos y parsearlos dentro de `main`
- mezclar configuracion y datos de negocio
- usar JSON para todo por costumbre
- no validar nunca el contenido importado

## Ruta de crecimiento buena

1. CSV + TOML
2. exportes mas serios
3. validaciones y errores tipados
4. SQLite
5. luego ya un backend con storage real

## Cruce recomendado

Este documento enlaza especialmente bien con:

- [docs/18_errores_io_y_anyhow.md](18_errores_io_y_anyhow.md)
- [docs/22_cargo_profesional_y_workspaces.md](22_cargo_profesional_y_workspaces.md)
- [docs/23_backend_local_con_axum_y_reqwest.md](23_backend_local_con_axum_y_reqwest.md)
- [docs/25_sqlite_con_rusqlite.md](25_sqlite_con_rusqlite.md)

## Cierre

Persistir bien no es cuestion de usar la tecnologia mas grande, sino la mas proporcionada al problema.
