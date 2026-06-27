# Serde, Clap y Thiserror

Este documento marca el paso del Rust base al Rust de aplicaciones reales.

## Por que estas crates importan

Cuando sales de los ejemplos pequenos, casi siempre necesitas:

- parsear entrada del usuario
- leer y escribir datos
- modelar errores con claridad

En Rust, tres crates muy comunes para eso son:

- `clap`
- `serde`
- `thiserror`

## `clap`

`clap` sirve para crear CLIs serias sin parsear argumentos a mano.

Ventajas:

- validacion de argumentos
- subcomandos
- ayuda automatica
- flags y opciones declarativas

Ejemplo mental:

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    comando: Comando,
}
```

Con eso describes la interfaz antes de implementarla.

## `serde`

`serde` sirve para serializar y deserializar tipos Rust.

Usos tipicos:

- JSON
- TOML
- YAML
- configuracion
- persistencia simple

Ejemplo:

```rust
#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    titulo: String,
}
```

Luego puedes convertir entre struct y JSON con `serde_json`.

## `thiserror`

`thiserror` ayuda a definir errores propios sin mucho boilerplate.

Ejemplo:

```rust
#[derive(Debug, Error)]
enum MiError {
    #[error("error de io: {0}")]
    Io(#[from] std::io::Error),
}
```

Ventajas:

- mensajes claros
- conversiones automáticas desde errores internos
- mejor modelado que `String` para todo

## Como se combinan

Un flujo muy tipico es:

1. `clap` interpreta los argumentos.
2. Tu programa llama a logica de negocio.
3. Esa logica usa `serde` para guardar o cargar datos.
4. Si algo falla, retorna errores tipados con `thiserror`.

## Donde verlo en este repo

Mira estos archivos:

- [src/bin/bonus_gestor_tareas_json.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/bin/bonus_gestor_tareas_json.rs)
- [src/lib.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/lib.rs)
- [data/bonus_tareas.json](/Users/legalintermedia/Documents/GitHub/RUST/data/bonus_tareas.json)

## Lecciones practicas

### Con `clap`

- deja que la libreria valide argumentos
- usa subcomandos para operaciones distintas
- evita parsear `env::args()` a mano salvo para aprender

### Con `serde`

- deriva `Serialize` y `Deserialize` cuando el tipo sea puro dato
- separa estructura interna y formato externo si hace falta
- usa JSON para empezar, no por obligación sino por simpleza

### Con `thiserror`

- define errores por dominio
- usa `#[from]` para encadenar errores de I/O o parseo
- no conviertas todo a `String` si el error tiene estructura real

## Antipatrones comunes

- meter toda la logica de CLI en `main`
- mezclar parseo de argumentos con logica de negocio
- guardar JSON construyendo strings a mano
- usar `unwrap()` en operaciones de archivo

## Arquitectura recomendable

Una forma sana de organizarlo es:

- binario: interfaz CLI
- libreria: logica y tipos
- datos: archivos de ejemplo

Eso justamente ya esta representado en este repositorio.

## Para seguir ampliando esta capa

Despues de este documento, enlaza bien con:

- [docs/18_errores_io_y_anyhow.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/18_errores_io_y_anyhow.md)
- [docs/22_cargo_profesional_y_workspaces.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/22_cargo_profesional_y_workspaces.md)
