# Tests de Integracion y Doctests

Cuando el curso entra en nivel intermedio, ya no basta con "tener algunos tests".

Conviene distinguir muy bien tres capas.

## Tests unitarios

Viven cerca del codigo, normalmente con `#[cfg(test)]`.

Son buenos para:

- funciones pequenas
- ramas concretas
- helpers internos

En este repo ya tienes varios en:

- [src/lib.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/lib.rs)

## Tests de integracion

Viven en `tests/` y prueban la API publica como si fueras un usuario de la libreria.

Eso obliga a comprobar dos cosas valiosas:

- que la API publica es suficiente
- que no dependes de detalles internos

Ejemplo anadido al repo:

- [tests/integracion_curso.rs](/Users/legalintermedia/Documents/GitHub/RUST/tests/integracion_curso.rs)

Lanzalo con:

```bash
cargo test --test integracion_curso
```

## Doctests

Son ejemplos en comentarios de documentacion que tambien se ejecutan.

Ventajas:

- la documentacion no miente
- el ejemplo de uso compila
- fuerzas una API publica mas clara

Ahora el repo ya incorpora varios doctests en:

- [src/lib.rs](/Users/legalintermedia/Documents/GitHub/RUST/src/lib.rs)

Puedes correrlos con:

```bash
cargo test --doc
```

## Regla de oro

Testea comportamiento, no implementacion.

Eso significa:

- entradas
- salidas
- efectos observables

No que una funcion use tal variable o tal bucle por dentro.

## Patron muy sano

Cuando una pieza cuesta mucho de testear, suele indicar una de estas cosas:

- mezcla I O y logica
- depende de estado global
- hace demasiadas cosas

Separar parseo puro de acceso a disco mejora mucho el panorama.

## `Result` en tests

Para tests con I O o temporales, devolver `Result<(), Box<dyn Error>>` suele quedar mas limpio que una cadena larga de `unwrap()`.

## Que merece la pena testear primero

1. parseo
2. reglas de negocio
3. persistencia minima
4. casos borde

## Casos borde que suelen faltar

- entrada vacia
- archivo vacio
- id inexistente
- repeticion de valores
- texto con espacios raros

## Anti patron muy comun

Escribir solo un test "happy path" y dar por cubierta la funcionalidad.

## Cierre

La mezcla de unit tests, integration tests y doctests te da mucha mas seguridad con muy poco coste adicional.
