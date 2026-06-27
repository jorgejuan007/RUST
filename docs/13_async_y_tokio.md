# Async y Tokio

Este documento no busca convertirte en experto en async, sino darte un mapa correcto para cuando llegues ahi.

## Que es `async`

`async` permite escribir operaciones que esperan I/O sin bloquear el hilo de la misma forma que el codigo sincronico tradicional.

Ejemplos tipicos:

- peticiones HTTP
- sockets
- lectura y escritura de red
- servicios web

## Que no es

No es "hacerlo todo mas rapido" por magia.

Tampoco sustituye por si solo:

- modelado correcto
- manejo de errores
- ownership

## Por que conviene llegar despues

Antes de estudiar `async`, deberias sentirte razonablemente comodo con:

- `Result`
- ownership
- referencias
- traits
- modulos

Si esas piezas ya te cuestan, `async` puede multiplicar la confusion.

## Futures en una frase

Una `Future` representa un trabajo que puede completarse mas tarde.

## `await`

`await` dice: "suspende aqui esta tarea hasta que el resultado este listo".

## El papel de Tokio

`tokio` es el runtime async mas popular del ecosistema Rust.

Te aporta:

- runtime
- timers
- sockets
- tasks async
- integracion con muchisimas crates

## Mapa de aprendizaje recomendado

1. Entender bien `Result`
2. Aprender `reqwest` basico
3. Ver una `async fn`
4. Entender `tokio::main`
5. Hacer un cliente HTTP simple
6. Pasar luego a `axum`

## Mini ejemplo conceptual

```rust
async fn saludar() {
    println!("hola");
}
```

Eso no se ejecuta solo. Necesitas un runtime que lo conduzca.

## Ejemplo ejecutable en este repo

Para ver un ejemplo pequeno pero real con `tokio`, ejecuta:

```bash
cargo run --bin bonus_async_tokio
```

Archivo relacionado:

- [src/bin/bonus_async_tokio.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/bin/bonus_async_tokio.rs)
- [docs/20_async_practico_con_tokio.md](/Users/legalintermedia/Documents/GitHub/RUST/docs/20_async_practico_con_tokio.md)

## Cuando usar async

Suele tener sentido cuando:

- tu app hace mucha I/O
- manejas muchas conexiones
- construyes APIs o servicios

## Cuando no hace falta

No hace falta de entrada para:

- CLIs pequenas
- utilidades locales simples
- scripts de procesamiento directo

## Consejo practico

No reescribas tu curso basico en async. Mejor haz primero una version sincrona clara y luego migra solo una parte concreta si aporta valor.
