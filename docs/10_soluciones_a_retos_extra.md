# Soluciones a los Retos Extra

Este documento completa los retos de `05_retos_extra_30_dias.md` con soluciones orientativas. No son las unicas validas, pero si son un buen punto de referencia.

## Semana 1

### Dia 1

```rust
fn main() {
    let a = 8.0;
    let b = 12.0;
    println!("Suma: {}", a + b);
    println!("Resta: {}", a - b);
    println!("Multiplicacion: {}", a * b);
    println!("Promedio: {}", (a + b) / 2.0);
}
```

### Dia 2

```rust
fn fahrenheit_a_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn area_triangulo(base: f64, altura: f64) -> f64 {
    base * altura / 2.0
}
```

### Dia 3

```rust
fn mediana_ordenado(valores: [i32; 5]) -> i32 {
    valores[2]
}
```

### Dia 4

```rust
fn minimo(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn signo(n: i32) -> &'static str {
    if n > 0 {
        "positivo"
    } else if n < 0 {
        "negativo"
    } else {
        "cero"
    }
}
```

### Dia 5

```rust
fn fibonacci(n: usize) {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        println!("{a}");
        let siguiente = a + b;
        a = b;
        b = siguiente;
    }
}
```

### Dia 6

```rust
fn clasificar(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vocal",
        '0'..='9' => "numero",
        'a'..='z' | 'A'..='Z' => "consonante",
        _ => "otro",
    }
}
```

### Dia 7

```rust
use std::io;

fn leer_numero() -> Result<f64, String> {
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .map_err(|e| e.to_string())?;
    entrada.trim().parse::<f64>().map_err(|_| "Numero invalido".to_string())
}
```

## Semana 2

### Dia 8

```rust
fn nombre_completo(nombre: &str, apellido: &str) -> String {
    format!("{nombre} {apellido}")
}
```

### Dia 9

```rust
fn imprimir(texto: &str) {
    println!("{texto}");
}
```

La idea del reto era eliminar un `clone()` innecesario y usar un prestamo.

### Dia 10

```rust
fn longitud(texto: &str) -> usize {
    texto.len()
}
```

### Dia 11

```rust
fn incrementar(valores: &mut [i32]) {
    for valor in valores {
        *valor += 1;
    }
}
```

### Dia 12

```rust
fn ultima_palabra(texto: &str) -> &str {
    texto.split_whitespace().last().unwrap_or("")
}
```

### Dia 13

```rust
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.alto > otro.alto
    }
}
```

### Dia 14

```rust
fn pendientes(tareas: &[Task]) -> usize {
    tareas.iter().filter(|t| !t.completada).count()
}
```

## Semana 3

### Dia 15

```rust
enum EstadoTarea {
    Pendiente,
    EnProgreso,
    Hecha,
    Bloqueada(String),
}
```

### Dia 16

```rust
fn buscar_tarea_mut(tareas: &mut [Task], id: u32) -> Option<&mut Task> {
    tareas.iter_mut().find(|t| t.id == id)
}
```

### Dia 17

```rust
fn parsear_numero(texto: &str) -> Result<i32, String> {
    let limpio = texto.trim();
    if limpio.is_empty() {
        return Err("La cadena esta vacia".to_string());
    }
    limpio
        .parse::<i32>()
        .map_err(|_| "La cadena no contiene un entero valido".to_string())
}
```

### Dia 18

```rust
use std::fs;
use std::io;

fn lineas_no_vacias(ruta: &str) -> Result<usize, io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    Ok(contenido.lines().filter(|l| !l.trim().is_empty()).count())
}
```

### Dia 19

```rust
fn normalizar(texto: &str) -> String {
    texto.to_lowercase()
}
```

### Dia 20

```rust
fn suma(valores: &[i32]) -> i32 {
    valores.iter().fold(0, |acc, n| acc + n)
}
```

### Dia 21

```rust
use std::collections::HashMap;

fn mas_frecuente(texto: &str) -> Option<(String, usize)> {
    let mut mapa = HashMap::new();
    for palabra in texto.split_whitespace() {
        *mapa.entry(palabra.to_string()).or_insert(0) += 1;
    }
    mapa.into_iter().max_by_key(|(_, cantidad)| *cantidad)
}
```

## Semana 4

### Dia 22

La solucion real del reto era estructural: mover funciones a archivos distintos como `parser.rs` y `stats.rs`, declararlos con `mod` y exponer solo lo necesario con `pub`.

### Dia 23

```rust
#[test]
fn cadena_vacia_tiene_cero_palabras() {
    assert_eq!(contar_palabras(""), 0);
}
```

### Dia 24

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
}
```

### Dia 25

```rust
trait Identificable {
    fn id(&self) -> u32;
}
```

### Dia 26

```rust
fn misma_longitud<T, U>(a: &[T], b: &[U]) -> bool {
    a.len() == b.len()
}
```

### Dia 27

```rust
struct Referencia<'a> {
    texto: &'a str,
}
```

### Dia 28

```rust
fn buscar(&self, id: u32) -> Option<&Task> {
    self.tareas.iter().find(|t| t.id == id)
}
```

## Semana 5

### Dia 29

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(format!("mensaje {i}")).unwrap();
        }
    });

    for mensaje in rx {
        println!("{mensaje}");
    }
}
```

### Dia 30

La solucion natural del reto era ampliar el proyecto final hacia una CLI con subcomandos. En este repositorio ya tienes una version bastante cercana en:

- `src/bin/bonus_gestor_tareas_std.rs`

## Cierre

Si tus soluciones se parecen en estructura aunque no en detalles exactos, vas bien. Lo importante es:

- modelar bien
- elegir firmas razonables
- evitar parches impulsivos
- explicar por que tu solucion es segura y clara
