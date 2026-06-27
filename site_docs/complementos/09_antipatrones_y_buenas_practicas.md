# Antipatrones y Buenas Practicas en Rust

Aprender Rust bien no es solo conocer lo correcto, sino tambien reconocer lo que suele salir mal.

## Antipatron: usar `clone()` como reflejo

### Problema

Copias datos para silenciar errores de ownership sin entender el problema.

### Mejor enfoque

Pregunta antes:

- necesito de verdad dos copias?
- podria prestar con `&`?
- la funcion deberia aceptar `&str` o `&[T]`?

## Antipatron: usar `String` para todo

### Problema

Pierdes flexibilidad y haces mas trabajo del necesario.

### Mejor enfoque

Recibe `&str` si solo lees. Guarda `String` si el tipo necesita poseer el texto.

## Antipatron: abusar de `unwrap()`

### Problema

Tu programa puede hacer `panic!` en errores previsibles.

### Mejor enfoque

Usa:

- `match`
- `if let`
- `unwrap_or`
- `unwrap_or_else`
- `?`
- `Result`

## Antipatron: convertir todo en mutable

### Problema

Pierdes claridad y haces mas dificil razonar sobre el codigo.

### Mejor enfoque

Empieza con inmutabilidad y abre mutacion solo donde sea necesaria.

## Antipatron: meter toda la logica en `main`

### Problema

El programa crece rapido y se vuelve inmantenible.

### Mejor enfoque

Extrae funciones, modulos y, cuando toque, una libreria.

## Antipatron: modelar estados con strings o bools dispersos

### Problema

Se cuelan estados invalidos y el codigo pierde claridad.

### Mejor enfoque

Usa enums para dominios finitos y estados.

## Antipatron: ignorar el compilador y parchear

### Problema

Arreglas sintomas, no la causa.

### Mejor enfoque

Lee el error completo y reduce el ejemplo si hace falta.

## Antipatron: funciones que poseen demasiado

### Problema

Tus APIs son rigidas y obligan a mover o clonar sin necesidad.

### Mejor enfoque

Disena firmas mas ligeras:

- `&str` en vez de `String` si solo lees
- `&[T]` en vez de `Vec<T>` si solo inspeccionas
- `&T` en vez de `T` si no necesitas ownership

## Antipatron: ignorar tests y `clippy`

### Problema

Pierdes feedback barato y rapido.

### Mejor enfoque

Acostumbrate a correr:

- `cargo check`
- `cargo fmt`
- `cargo clippy`
- `cargo test`

## Antipatron: luchar con lifetimes demasiado pronto

### Problema

Te bloqueas intentando dominar un tema avanzado antes de entender ownership y borrowing.

### Mejor enfoque

Aprende primero:

- moves
- prestamos
- `String` vs `&str`
- referencias en retornos simples

## Buenas practicas que si conviene cultivar

### Funciones pequenas

Cada funcion deberia tener una responsabilidad clara.

### Tipos honestos

Si algo puede fallar, usa `Result`. Si puede faltar, usa `Option`.

### Modelado explicito

Usa `struct` para entidades y `enum` para variantes o estados.

### Parametros flexibles

Acepta prestamos cuando puedas.

### Refactor temprano

Cuando un archivo crece mucho, separa modulos antes de que duela.

### Nombres claros

Rust se beneficia mucho de nombres directos y especificos.

## Regla de cierre

Si tu codigo:

- necesita menos `clone()`
- usa menos `unwrap()`
- modela mejor con enums
- separa mejor la logica

entonces casi seguro estas mejorando de verdad.
