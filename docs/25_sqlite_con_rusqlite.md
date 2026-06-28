# SQLite con `rusqlite`

Cuando CSV y TOML se quedan cortos, SQLite suele ser el siguiente escalon mas razonable.

## Por que SQLite encaja tan bien

Te aporta varias cosas a la vez:

- persistencia real en un archivo
- consultas mas ricas
- updates y filtros con SQL
- cero servidor externo

Para aprendizaje y herramientas locales, eso es una combinacion muy potente.

## `rusqlite` en una frase

`rusqlite` es un wrapper ergonomico sobre SQLite para Rust.

Sirve muy bien para:

- CLIs con almacenamiento local
- prototipos de producto
- capas de persistencia pequenas
- ejercicios de modelado y consultas

## Demo ejecutable del repo

Archivo:

- [src/bin/bonus_sqlite_rusqlite.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/bin/bonus_sqlite_rusqlite.rs)

Ejecuta:

```bash
cargo run --bin bonus_sqlite_rusqlite
```

Ese bonus:

- crea una base local en `target/bonus_tasks.sqlite3`
- inicializa el esquema
- inserta tareas
- actualiza una fila
- consulta tareas y pendientes
- muestra como filtrar por estado

## Que piezas conviene mirar

### Esquema explicito

La tabla se crea desde Rust con `CREATE TABLE IF NOT EXISTS`.

Eso te obliga a pensar en:

- columnas
- tipos
- claves
- modelo de estados

### `params!`

El macro `params!` evita interpolar strings a mano y deja consultas mas seguras y legibles.

### `query_map`

Es un patron muy comun para transformar filas SQL en structs Rust.

La clave no es memorizarlo, sino entender el flujo:

1. preparas consulta
2. iteras filas
3. conviertes cada fila a tu tipo

## Cuando usarlo de verdad

SQLite encaja muy bien cuando:

- la app es local
- no necesitas concurrencia distribuida
- quieres persistencia mas seria que archivos planos
- el dominio aun cabe en pocas tablas

## Anti patrones comunes

- meter SQL inline por todas partes sin separar funciones
- usar strings para construir consultas
- no manejar errores de apertura o esquema
- mezclar acceso a BD y presentacion en la misma funcion

## Ruta de crecimiento buena

1. schema pequeno
2. helpers de insercion y lectura
3. funciones de busqueda y conteo
4. separar storage del binario
5. luego conectar esa storage a un backend

## Cruce recomendado

Este documento encaja muy bien con:

- [docs/24_persistencia_con_csv_y_toml.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/24_persistencia_con_csv_y_toml.md)
- [docs/26_backend_axum_modular_y_storage.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/26_backend_axum_modular_y_storage.md)

## Cierre

SQLite es un punto medio excelente: mucho mas serio que un CSV, mucho mas simple que montar un servidor de base de datos.
