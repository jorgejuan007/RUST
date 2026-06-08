# Chuleta de Rust y Cargo

Este archivo es para consulta rapida mientras practicas.

## Comandos Cargo mas usados

```bash
cargo new nombre_proyecto
cargo run
cargo run --bin dia_01_hola_rust
cargo check
cargo check --bins
cargo test
cargo fmt
cargo clippy
```

## Variable inmutable y mutable

```rust
let x = 10;
let mut y = 20;
y += 1;
```

## Shadowing

```rust
let x = 5;
let x = x + 1;
```

## Tipos basicos

```rust
let a: i32 = 10;
let b: f64 = 2.5;
let c: bool = true;
let d: char = 'R';
```

## Tuplas y arrays

```rust
let persona = ("Ana", 30);
let numeros = [1, 2, 3, 4];
```

## Funcion simple

```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}
```

## If como expresion

```rust
let mayor = if a > b { a } else { b };
```

## Match

```rust
match valor {
    0 => println!("cero"),
    1..=10 => println!("pequeno"),
    _ => println!("otro"),
}
```

## String y &str

```rust
let s1 = String::from("hola");
let s2: &str = "mundo";
```

## Prestamo

```rust
fn ver(texto: &str) {
    println!("{texto}");
}
```

## Prestamo mutable

```rust
fn editar(texto: &mut String) {
    texto.push('!');
}
```

## Struct e impl

```rust
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }
}
```

## Enum

```rust
enum Estado {
    Pendiente,
    Hecha,
}
```

## Option

```rust
fn buscar() -> Option<i32> {
    Some(42)
}
```

## Result

```rust
fn parsear(texto: &str) -> Result<i32, String> {
    texto.parse::<i32>().map_err(|_| "error".to_string())
}
```

## Operador ?

```rust
use std::fs;
use std::io;

fn leer(ruta: &str) -> Result<String, io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    Ok(contenido)
}
```

## Vec

```rust
let mut v = vec![1, 2, 3];
v.push(4);
```

## HashMap

```rust
use std::collections::HashMap;

let mut mapa = HashMap::new();
mapa.insert("ana", 30);
```

## Iteradores

```rust
let resultado: Vec<i32> = numeros
    .iter()
    .filter(|n| **n % 2 == 0)
    .map(|n| *n * 2)
    .collect();
```

## Trait

```rust
trait Resumen {
    fn resumen(&self) -> String;
}
```

## Generico

```rust
fn maximo<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

## Lifetime simple

```rust
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## Test unitario

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_suma() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Modulo

```rust
mod matematicas {
    pub fn sumar(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

## Patron de firma idiomatica

Usa estas firmas como referencia mental:

```rust
fn leer_texto(texto: &str) {}
fn editar_texto(texto: &mut String) {}
fn buscar_por_id(items: &[Item], id: u32) -> Option<&Item> { todo!() }
fn cargar(ruta: &str) -> Result<String, std::io::Error> { todo!() }
```

## Regla rapida de decision

- Si solo lees texto: `&str`
- Si posees texto: `String`
- Si solo lees una lista: `&[T]`
- Si modificas una lista: `&mut Vec<T>` o `&mut [T]`
- Si puede faltar un valor: `Option<T>`
- Si puede fallar con explicacion: `Result<T, E>`
