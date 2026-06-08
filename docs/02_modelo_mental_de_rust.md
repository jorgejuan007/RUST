# Modelo Mental de Rust

Rust se vuelve mucho mas amable cuando entiendes su modelo mental. Este documento no busca darte definiciones academicas, sino intuiciones correctas.

## La idea central

Rust quiere darte:

- velocidad de lenguaje de sistemas
- seguridad de memoria
- concurrencia mas fiable

Y lo hace moviendo muchos problemas del tiempo de ejecucion al tiempo de compilacion.

## Ownership como responsabilidad

No pienses ownership como una "restriccion rara". Piensalo como responsabilidad sobre un valor.

Si un valor tiene un duenio:

- alguien decide cuando vive
- alguien decide cuando deja de existir
- el compilador puede evitar dobles liberaciones y usos invalidos

## Stack y heap sin dolor

Una simplificacion util:

- valores pequenos y de tamano conocido suelen vivir en stack
- datos dinamicos como `String` y `Vec` gestionan memoria en heap

No necesitas dominar implementaciones internas para empezar. Lo importante es:

- copiar un `i32` suele ser barato
- mover un `String` transfiere control sobre su buffer

## Copy, move y clone

### Copy

Tipos simples como `i32`, `bool` o `char` suelen copiarse implicitamente.

```rust
let a = 5;
let b = a;
```

`a` sigue siendo usable porque copiarlo es barato y seguro.

### Move

Con `String`, `Vec` y muchos structs, asignar o pasar a funciones suele mover.

```rust
let a = String::from("hola");
let b = a;
```

Aqui `a` deja de ser usable porque la propiedad del valor paso a `b`.

### Clone

`clone()` duplica datos de verdad.

```rust
let a = String::from("hola");
let b = a.clone();
```

Usalo cuando quieras dos valores independientes. No lo uses para silenciar al compilador sin pensar.

## Borrowing como prestamo temporal

Prestas cuando quieres usar un valor sin tomarlo.

### Prestamo inmutable

```rust
fn ver(texto: &str) {
    println!("{texto}");
}
```

Permite leer sin modificar.

### Prestamo mutable

```rust
fn editar(texto: &mut String) {
    texto.push('!');
}
```

Permite modificar, pero exige exclusividad temporal.

## Regla clave del borrowing

En un mismo momento puedes tener:

- muchas referencias inmutables
- o una mutable

pero no ambas cosas a la vez sobre el mismo dato.

Rust hace esto para evitar:

- leer mientras otro cambia
- aliasing peligroso
- carreras de datos

## String vs &str

Piensa asi:

- `String` es texto poseido, editable, dinamico
- `&str` es vista prestada de texto

Usa `&str` cuando:

- solo necesitas leer
- quieres una firma flexible

Usa `String` cuando:

- necesitas construir o modificar
- tu tipo debe poseer el texto

## Slices

Un slice es una vista sobre parte de una coleccion.

Ejemplos:

- `&str`
- `&[i32]`

Ventajas:

- no copian
- expresan "quiero mirar una parte"
- hacen APIs mas ligeras

## Option como presencia o ausencia

Rust no depende de `null` para casi nada. En su lugar usa:

```rust
Option<T>
```

Piensa:

- `Some(valor)` = hay dato
- `None` = no hay dato

Cuando una busqueda puede fallar o un valor puede faltar, `Option` suele ser la mejor herramienta.

## Result como exito o error

Para operaciones que pueden fallar de forma explicable:

```rust
Result<T, E>
```

Piensa:

- `Ok(valor)` = exito
- `Err(error)` = fallo recuperable

`Result` te fuerza a decidir que hacer con el error, en vez de esconderlo.

## Structs para entidades

Usa `struct` cuando un valor siempre tiene la misma forma.

```rust
struct Usuario {
    nombre: String,
    edad: u32,
}
```

Piensa en `struct` como "una entidad con campos coherentes".

## Enums para estados o variantes

Usa `enum` cuando un valor puede ser una de varias formas validas.

```rust
enum Estado {
    Pendiente,
    Hecha,
}
```

Rust te empuja a modelar bien el dominio. En muchos lenguajes esto se resuelve con:

- strings magicos
- enteros de estado
- varios bools

En Rust casi siempre `enum` es mejor.

## Match como herramienta de modelado

`match` no es solo un `switch`. Es una forma de obligarte a pensar en todos los casos.

Eso te da dos beneficios:

- menos olvidos
- codigo mas honesto con el dominio

## Iteradores como estilo expresivo

Con iteradores puedes describir transformaciones paso a paso:

```rust
let pares: Vec<i32> = numeros
    .iter()
    .filter(|n| **n % 2 == 0)
    .map(|n| *n * 10)
    .collect();
```

Piensa en ellos como tuberias de datos.

## Traits como comportamiento compartido

Un trait dice:

"estos tipos pueden hacer esto"

Ejemplos comunes:

- `Debug`
- `Clone`
- `PartialEq`
- traits propios como `Resumen`

Los traits son una pieza central del diseno idiomatico en Rust.

## Genericos como abstraccion segura

Un generico permite escribir logica reusable sin perder chequeo de tipos.

```rust
fn maximo<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

No escribes "para cualquier cosa". Escribes "para cualquier tipo que cumpla estas reglas".

## Lifetimes sin misterio

Los lifetimes no "alargan" la vida de nada. Solo describen relaciones entre referencias.

Piensa asi:

- si devuelvo una referencia
- esa referencia tiene que apuntar a algo que siga vivo

Eso es todo el problema que los lifetimes ayudan a expresar.

## Concurrencia segura

Rust no te regala concurrencia facil, te da concurrencia mas honesta.

Te obliga a explicitar:

- quien posee que
- que se comparte
- como se sincroniza

Eso reduce errores tipicos de programas concurrentes.

## Async como siguiente capa

No estudies `async` demasiado pronto. Primero domina:

- ownership
- Result
- traits
- modulos

Luego `async` sera mas razonable.

## Preguntas de control

Si quieres comprobar si ya vas pensando en Rust, intenta responder sin mirar:

- cuando debo usar `String` y cuando `&str`?
- cuando una funcion deberia devolver `Option`?
- cuando una deberia devolver `Result`?
- cuando conviene un `enum` frente a varios `bool`?
- por que un `clone()` puede ser una mala senal?

## Resumen de una frase por concepto

- ownership: un valor tiene un responsable
- borrowing: puedo usar sin aduenarme
- mutable borrow: puedo editar con exclusividad
- `Option`: puede no haber dato
- `Result`: puede haber error
- `struct`: entidad con forma estable
- `enum`: conjunto de variantes posibles
- `match`: cubrir todos los casos
- trait: comportamiento compartido
- generic: logica reusable con restricciones

Si estas frases ya te suenan naturales, vas muy bien.
