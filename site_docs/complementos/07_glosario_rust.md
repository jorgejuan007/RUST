# Glosario de Rust

Este glosario resume conceptos clave del lenguaje con explicaciones cortas y practicas.

## Terminos

### API

Conjunto de funciones, tipos y comportamientos que expones para usar un modulo o libreria.

### Borrowing

Prestamo temporal de un valor sin transferir ownership.

### Bound

Restriccion sobre un tipo generico, normalmente expresada con traits.

### Cargo

Herramienta oficial para crear, compilar, probar y gestionar dependencias en proyectos Rust.

### Closure

Funcion anonima que puede capturar variables del entorno.

### Copy

Trait que permite que ciertos tipos se copien implicitamente al asignarlos o pasarlos.

### Crate

Unidad de compilacion en Rust. Puede ser binaria o de biblioteca.

### Enum

Tipo algebraico que representa un valor que puede tomar varias variantes bien definidas.

### Expression

Fragmento de codigo que produce un valor.

### Function

Bloque reutilizable de codigo definido con `fn`.

### Generic

Parametro de tipo que permite reutilizar logica en distintos tipos manteniendo seguridad.

### Heap

Zona de memoria usada normalmente por datos dinamicos como `String` y `Vec`.

### Lifetime

Relacion entre referencias que ayuda al compilador a comprobar que no apuntan a datos muertos.

### Macro

Herramienta de metaprogramacion como `println!` o `vec!`.

### Match

Construccion de control que permite cubrir patrones de forma exhaustiva.

### Module

Mecanismo de organizacion del codigo dentro de una crate.

### Move

Transferencia de ownership de un valor a otra variable o funcion.

### Mutable reference

Referencia exclusiva que permite modificar el valor apuntado.

### Ownership

Regla central por la que cada valor tiene un duenio responsable de su ciclo de vida.

### Pattern matching

Forma de desestructurar y analizar valores a traves de patrones.

### Result

Tipo estandar para representar exito (`Ok`) o error (`Err`).

### Scope

Region del codigo donde un nombre o valor sigue siendo valido.

### Slice

Vista prestada sobre parte de una coleccion, por ejemplo `&str` o `&[i32]`.

### Stack

Zona de memoria usada normalmente para datos pequenos o de tamano fijo.

### Statement

Instruccion que realiza una accion pero no devuelve valor directamente.

### String

Texto UTF-8 poseido, dinamico y mutable.

### Struct

Tipo compuesto con campos nombrados.

### Trait

Contrato de comportamiento compartido entre tipos.

### Type inference

Capacidad del compilador para deducir tipos sin anotarlos siempre.

### Unwrap

Metodo que extrae el valor de `Option` o `Result`, pero hace `panic!` si no hay exito.

### Vec

Vector dinamico de elementos del mismo tipo.

### `&str`

Slice de texto UTF-8 prestado.

## Regla practica del glosario

Si puedes explicar con tus palabras al menos estos diez terminos, ya tienes una base muy buena:

- ownership
- borrowing
- move
- `String`
- `&str`
- `Option`
- `Result`
- trait
- enum
- lifetime
