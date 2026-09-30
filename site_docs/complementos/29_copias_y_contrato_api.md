# Copias recuperables y contrato HTTP

La persistencia necesita un camino de recuperación y una descripción comprobable de
la API. Este bloque amplía el [proyecto final](28_proyecto_tareas_persistente.md).

## Guardar una copia

```bash
cargo run --bin proyecto_tareas_cli -- --db ./mis-tareas.sqlite3 backup ./copias/tareas.backup.json
```

El comando captura todas las tareas, sus estados y el último ID asignado en una misma
transacción. Puede ejecutarse mientras el servidor usa la base: SQLite mantiene una
vista consistente de los datos. El límite es de 10 000 tareas y 16 MiB de JSON.

La copia se escribe y sincroniza en un archivo temporal de la carpeta destino antes de
guardarla con el nombre elegido. Si ese nombre ya existe, el comando falla sin
sobrescribirlo. Usa otro nombre para la siguiente copia. Los errores devuelven un código
de salida distinto de cero.

No copies solo el archivo `.sqlite3` de una base abierta: este proyecto usa WAL y los
datos recientes pueden estar en archivos auxiliares. La copia JSON consulta SQLite.

## Recuperar y comprobar

```bash
cargo run --bin proyecto_tareas_cli -- --db ./recuperadas.sqlite3 restore ./copias/tareas.backup.json
cargo run --bin proyecto_tareas_cli -- --db ./recuperadas.sqlite3 list
cargo run --bin proyecto_tareas_cli -- --db ./recuperadas.sqlite3 stats
```

El destino debe estar sin tareas ni historial de IDs: usa una ruta nueva. Una base
que contuvo tareas sigue teniendo historial aunque hayas borrado todas. La restauración
rechaza ese destino y conserva sus datos. Si quieres que el servidor use la base
recuperada, detenlo con Ctrl+C y arráncalo con `--db ./recuperadas.sqlite3`.

La validación comprueba versión, títulos recortados de 1–200 caracteres Unicode, IDs
positivos únicos y un `ultimo_id` suficiente. No admite NUL en los títulos. Rechaza campos desconocidos, JSON
incompleto y archivos demasiado grandes. La inserción y el ajuste de la secuencia se
realizan en una transacción de escritura: un fallo revierte toda la restauración.

El formato tiene su propia versión:

```json
{
  "version": 1,
  "ultimo_id": 3,
  "tareas": [
    {"id": 1, "titulo": "Aprender Rust", "hecha": true}
  ]
}
```

Aquí la siguiente tarea recibirá el ID 4. Los IDs 2 y 3 pueden corresponder a tareas
borradas; siguen reservados. El archivo JSON del bonus inicial es otro formato y no
se importa automáticamente.

Puedes usar la misma funcionalidad desde Rust con
[TaskStore::snapshot y TaskStore::restore](https://github.com/jorgejuan007/RUST/blob/main/src/api/storage.rs), y gestionar archivos
con [Backup::read y Backup::write_new](https://github.com/jorgejuan007/RUST/blob/main/src/api/backup.rs).

## Contrato OpenAPI

El [contrato descargable](../assets/openapi.json) describe rutas, métodos, parámetros,
modelos JSON y códigos de error. Usa [OpenAPI 3.1.1](https://spec.openapis.org/oas/v3.1.1.html)
y se sirve en la misma dirección que la API:

```bash
curl -sS http://127.0.0.1:3000/openapi.json
```

Puedes importarlo en una herramienta compatible con OpenAPI y configurar la dirección
de tu servidor. El contrato usa un servidor relativo `/`, por lo que no fija un puerto.
La API sigue siendo un proyecto local sin autenticación.

Las peticiones de creación y edición requieren `Content-Type: application/json`.
Los títulos se validan después de recortar espacios; `null` en un campo de edición
equivale a omitirlo y debe haber al menos otro cambio efectivo. Los ejemplos del
contrato incluyen estos casos.

## Verificación automática

Instala las dependencias de `requirements-docs.txt` y ejecuta:

```bash
make test-contract
cargo test --test copias_tareas
```

El primer comando valida la estructura OpenAPI y sus ejemplos, arranca un servidor
en un puerto libre con una base temporal y comprueba 25 respuestas HTTP contra los
esquemas: creación, consulta, edición, paginación, borrado y errores. Comprueba también
que el documento servido coincide con el archivo del repositorio. No usa tus tareas.

Las pruebas de copias cubren recuperación, Unicode, historial de IDs, rechazo de
archivos existentes, límites, reversión de fallos y dos restauraciones concurrentes.
Ambas comprobaciones forman parte de `make ci` y de GitHub Actions.

Para validar solo el contrato, sin arrancar la API:

```bash
python3 scripts/check_openapi.py
```
