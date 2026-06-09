# Evaluacion Final y Rubrica

Este documento te ayuda a cerrar el curso con una prueba seria y una autoevaluacion honesta.

## Parte 1. Conceptos

Intenta responder por escrito:

1. Que diferencia hay entre `String` y `&str`?
2. Que significa ownership?
3. Que diferencia hay entre `Option` y `Result`?
4. Cuando usarias `enum` en vez de varios `bool`?
5. Para que sirve `match`?
6. Que hace `?`?
7. Que es un trait?
8. Que resuelven los genericos?
9. Que expresan los lifetimes?
10. Que valor te aporta `cargo` en el dia a dia?

## Parte 2. Practica minima obligatoria

Implementa desde cero, sin mirar, un programa que:

- lea un archivo de texto
- cuente palabras
- muestre la palabra mas frecuente
- devuelva `Result`
- tenga al menos un test

## Parte 3. Proyecto final

Construye una CLI de tareas o notas que soporte:

- agregar
- listar
- completar o marcar
- borrar
- persistir datos en archivo
- manejar errores sin `panic!` innecesario

## Rubrica

### Nivel 1. Base inicial

- compilas ejemplos simples
- entiendes variables, funciones y control de flujo
- todavia te bloqueas con ownership

### Nivel 2. Base funcional

- ya manejas `String` y `&str`
- ya usas `Option` y `Result`
- puedes resolver ejercicios pequenos sin mirar mucho

### Nivel 3. Rust competente inicial

- modelas con `struct` y `enum`
- entiendes referencias y mutabilidad
- separas codigo en funciones y modulos
- escribes tests sencillos

### Nivel 4. Rust practico

- construyes CLIs pequenas y medianas
- eliges firmas razonables
- usas iteradores y colecciones con naturalidad
- lees errores del compilador con calma

### Nivel 5. Listo para siguiente nivel

- puedes arrancar proyectos propios
- ya no abusas de `clone()` ni `unwrap()`
- piensas en terminos de modelado y API
- estas listo para aprender crates como `serde`, `clap`, `tokio` o `axum`

## Criterios concretos para darte por aprobado

Date por aprobado si puedes hacer estas ocho cosas:

1. explicar ownership con un ejemplo real
2. escribir una funcion que reciba `&str`
3. devolver `Result` desde una funcion de I/O
4. modelar estados con un enum
5. escribir un test unitario
6. separar un archivo en modulos
7. leer un error de move sin colapsar
8. terminar un mini proyecto propio

## Si no llegas aun

No pasa nada. Repite especialmente:

- dias 8 a 12
- dias 16 a 20
- dias 22 a 28

Esas tres zonas suelen marcar la diferencia.
