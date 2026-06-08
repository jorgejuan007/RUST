# Diagnostico de Errores en Rust

La habilidad mas rentable en Rust no es memorizar sintaxis. Es aprender a leer bien al compilador.

## Como leer un error

Normalmente conviene seguir este orden:

1. Lee el encabezado del error.
2. Mira la linea marcada.
3. Busca donde nacio el prestamo, movimiento o tipo original.
4. Lee la sugerencia del compilador.
5. Reduce el ejemplo si sigues confundido.

## Error: borrow of moved value

Significa que moviste un valor y despues quisiste reutilizarlo.

### Ejemplo

```rust
let s = String::from("hola");
let t = s;
println!("{s}");
```

### Que esta pasando

`String` no es `Copy`. Al hacer `let t = s;`, la propiedad pasa a `t`.

### Formas comunes de arreglarlo

- presta con `&s`
- clona con `s.clone()` si necesitas dos valores independientes
- reorganiza el flujo para no reutilizar el valor despues del move

## Error: cannot borrow as mutable because it is also borrowed as immutable

Significa que hay una referencia mutable conviviendo con una inmutable sobre el mismo dato.

### Ejemplo

```rust
let mut s = String::from("hola");
let r1 = &s;
let r2 = &mut s;
println!("{r1} {r2}");
```

### Idea clave

Rust quiere evitar leer y modificar el mismo dato simultaneamente.

### Soluciones tipicas

- acorta el alcance de `r1`
- usa primero la referencia inmutable y luego crea la mutable
- separa el codigo en bloques

## Error: expected &str, found String

Muy comun cuando una firma espera texto prestado y tu pasas texto poseido o viceversa.

### Ejemplo

```rust
fn ver(texto: &str) {}

let s = String::from("hola");
ver(s);
```

### Arreglo

```rust
ver(&s);
```

### Regla mental

Si una funcion solo lee texto, casi siempre quiere `&str`.

## Error: mismatched types

Suele indicar que una rama de `if`, `match` o el retorno no coincide con lo esperado.

### Ejemplo

```rust
fn ejemplo(cond: bool) -> i32 {
    if cond {
        10
    } else {
        "hola"
    }
}
```

### Solucion

Todas las ramas deben devolver el mismo tipo.

## Error: cannot return reference to local variable

Estas intentando devolver una referencia a un valor que muere al salir de la funcion.

### Ejemplo

```rust
fn mala() -> &str {
    let s = String::from("hola");
    &s
}
```

### Arreglo conceptual

- devuelve un valor poseido, por ejemplo `String`
- o devuelve una referencia a algo que vino de fuera

## Error: temporary value dropped while borrowed

Se produce cuando la referencia apunta a un valor temporal que desaparece enseguida.

### Ejemplo

```rust
let r = &String::from("hola");
```

Aunque a veces el compilador te permita casos concretos, la intuicion correcta es: cuidado con referenciar temporales.

## Error: trait bound not satisfied

El compilador te esta diciendo que un tipo no cumple una capacidad exigida.

### Ejemplo

```rust
fn mostrar<T: std::fmt::Debug>(x: T) {
    println!("{x:?}");
}
```

Si el tipo no implementa `Debug`, fallara.

### Arreglo

- implementa el trait
- deriva el trait si aplica, por ejemplo `#[derive(Debug)]`
- relaja la firma si no necesitabas ese bound

## Error: no method named ...

Puede significar varias cosas:

- olvidaste importar un trait
- el tipo no tiene ese metodo
- estas llamando sobre una referencia equivocada

## Error: use of undeclared crate or module

Normalmente aparece por:

- modulo no declarado con `mod`
- ruta incorrecta
- dependencia faltante en `Cargo.toml`

## Error: type annotations needed

El compilador no tiene suficiente contexto para inferir un tipo.

### Solucion tipica

Anota el tipo en la variable o en el parseo:

```rust
let n: i32 = "42".parse().unwrap();
```

## Debugging practico

Usa estas herramientas antes de cambiar medio programa:

- `println!`
- `dbg!`
- `cargo check`
- tests pequenos

## Flujo recomendado de correccion

1. Haz el ejemplo mas pequeno posible.
2. Pregunta si el problema es de ownership, mutabilidad o tipos.
3. Cambia una sola cosa.
4. Recompila.
5. Si mejora, avanza; si no, retrocede y prueba otra hipotesis.

## Preguntas de rescate rapido

Cuando algo no compila, preguntate:

- quien posee este valor?
- esta referencia sigue viva demasiado tiempo?
- estoy intentando mutar mientras leo?
- esta funcion deberia aceptar `&str`?
- deberia devolver `Option` o `Result`?

## Senales de parche peligroso

Estas resolviendo mal el problema si empiezas a:

- meter `clone()` por todas partes
- cambiar todo a `String` sin criterio
- usar `unwrap()` en cualquier sitio
- hacer mutable medio programa por desesperacion

## Lo importante

En Rust, el error del compilador no es un enemigo. Casi siempre te esta senalando una inconsistencia real entre lo que tu codigo hace y lo que dice que quiere hacer.
