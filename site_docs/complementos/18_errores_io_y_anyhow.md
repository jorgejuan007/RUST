# Errores, I O y `anyhow`

Esta capa es una de las diferencias reales entre "compila" y "sirve para una aplicacion".

## La idea central

En Rust conviene separar dos preguntas:

- como represento un error en mi dominio
- como propago y contextualizo errores en la aplicacion

Normalmente:

- libreria o modulo reusable: errores tipados
- binario o CLI: ergonomia y contexto

## I O basico que usaras mucho

Lo mas frecuente al principio es esto:

- `std::fs::read_to_string`
- `std::fs::write`
- `std::path::Path`
- `std::env::args`

Ese conjunto ya te deja construir:

- CLIs que leen archivos
- importadores de datos
- configuraciones simples
- exportadores a texto o JSON

## `Result` primero, `panic!` casi nunca

Si una operacion puede fallar por el entorno, deberia devolver `Result`.

Ejemplos tipicos:

- archivo inexistente
- permisos insuficientes
- texto mal formateado
- parseo numerico invalido

`panic!` tiene sentido para invariantes rotas, no para errores normales de entrada.

## `thiserror` vs `anyhow`

Regla practica:

- `thiserror` cuando disenas errores propios con forma estable
- `anyhow` cuando quieres propagar errores con poco boilerplate desde una aplicacion

No compiten entre si. Se complementan.

## El valor real de `Context`

`anyhow` destaca cuando anades contexto:

```rust
let contenido = fs::read_to_string(ruta)
    .with_context(|| format!("no se pudo leer {}", ruta.display()))?;
```

Eso evita mensajes pobres del tipo "invalid digit found in string" sin ninguna pista de donde paso.

## Flujo recomendable

1. lee el archivo
2. valida que tiene contenido util
3. parsea linea a linea o bloque a bloque
4. devuelve errores con contexto
5. separa parseo puro de I O real

Esa ultima parte importa mucho porque vuelve tu codigo testeable.

## Ejemplo ejecutable en este repo

Puedes verlo aqui:

- src/bin/bonus_errores_io_anyhow.rs (`src/bin/bonus_errores_io_anyhow.rs`)
- data/bonus_numeros.txt (`data/bonus_numeros.txt`)

Prueba:

```bash
cargo run --bin bonus_errores_io_anyhow
```

Y tambien con una ruta manual:

```bash
cargo run --bin bonus_errores_io_anyhow -- data/bonus_numeros.txt
```

## Patron sano

- una funcion pura que parsea texto
- una funcion de I O que lee del disco
- `main` muy pequeno que conecta todo

Eso es justo lo que hace el bonus nuevo.

## Patron peligroso

- leer archivo
- parsear
- imprimir
- validar
- manejar errores

todo mezclado dentro de `main`.

Funciona para demos pequenas, pero escala mal y se testea peor.

## Donde encaja `thiserror`

En este mismo repositorio ya tienes un ejemplo mas estructurado con:

- src/lib.rs (`src/lib.rs`)
- src/bin/bonus_gestor_tareas_json.rs (`src/bin/bonus_gestor_tareas_json.rs`)

Ahi `TaskStoreError` modela errores concretos del almacenamiento JSON.

## Reglas rapidas de firma

- si puede fallar: `fn ... -> Result<T, E>`
- si consumes una ruta: `impl AsRef<Path>`
- si parseas texto externo: asume que vendra roto alguna vez

## Antipatrones comunes

- `unwrap()` en operaciones de archivo
- convertir todos los errores a `String`
- esconder la causa real del fallo
- devolver `Option` cuando en realidad quieres explicar el error

## Ejercicio muy recomendable

Extiende el bonus para:

- ignorar tambien lineas con comentarios
- rechazar numeros negativos con `bail!`
- exportar el resumen a un segundo archivo

## Cierre

Si controlas bien `Result`, `Path`, `fs` y `anyhow`, ya puedes construir mucha mas herramienta util de la que parece.
