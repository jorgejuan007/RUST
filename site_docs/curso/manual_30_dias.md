# Manual Practico de Rust en 30 Dias

Este manual convierte el plan de 30 dias en trabajo concreto. Cada dia incluye:

- un enunciado claro
- una solucion guia en Rust
- una rubrica corta para autoevaluarte

Regla de estudio recomendada:

1. Intenta resolver el ejercicio sin mirar la solucion.
2. Compara tu version con la solucion guia.
3. Reescribe la solucion con tus propias palabras.
4. Ejecuta `cargo fmt`, `cargo clippy` y, cuando aplique, `cargo test`.

Documentacion complementaria recomendada:

- `docs/01_guia_de_estudio_y_habitos.md`
- `docs/02_modelo_mental_de_rust.md`
- `docs/03_chuleta_rust_y_cargo.md`
- `docs/04_diagnostico_de_errores_rust.md`
- `docs/05_retos_extra_30_dias.md`
- `docs/06_proyectos_finales_y_siguiente_nivel.md`
- `docs/07_glosario_rust.md`
- `docs/08_preguntas_de_repaso_y_respuestas.md`
- `docs/09_antipatrones_y_buenas_practicas.md`
- `docs/10_soluciones_a_retos_extra.md`
- `docs/11_evaluacion_final_y_rubrica.md`
- `docs/12_serde_clap_thiserror.md`
- `docs/13_async_y_tokio.md`
- `docs/14_arquitectura_y_refactor_en_rust.md`
- `docs/15_ruta_postcurso.md`
- `docs/16_soluciones_completas_ejecutables.md`
- `docs/17_automatizacion_y_publicacion.md`
- `docs/18_errores_io_y_anyhow.md`
- `docs/19_smart_pointers_y_mutabilidad_interior.md`
- `docs/20_async_practico_con_tokio.md`
- `docs/21_tests_de_integracion_y_doctests.md`
- `docs/22_cargo_profesional_y_workspaces.md`

## Dia 1. Hola Rust y Cargo

**Enunciado**

Crea un proyecto nuevo y escribe un programa que imprima un saludo y luego sume dos numeros enteros fijos.

**Solucion guia**

```rust
fn main() {
    let a = 8;
    let b = 12;
    let suma = a + b;

    println!("Hola, Rust!");
    println!("{} + {} = {}", a, b, suma);
}
```

**Rubrica**

- Sabes crear el proyecto con `cargo new`.
- Sabes ejecutar con `cargo run`.
- Entiendes que `main` es el punto de entrada.
- Entiendes que `println!` es una macro.

## Dia 2. Variables, mutabilidad y tipos

**Enunciado**

Escribe un conversor de grados Celsius a Fahrenheit y calcula tambien el area de un rectangulo. Usa funciones separadas.

**Solucion guia**

```rust
fn celsius_a_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn area_rectangulo(base: f64, altura: f64) -> f64 {
    base * altura
}

fn main() {
    let temperatura_c: f64 = 25.0;
    let base = 5.0;
    let altura = 3.0;

    let temperatura_f = celsius_a_fahrenheit(temperatura_c);
    let area = area_rectangulo(base, altura);

    println!("{temperatura_c}C = {temperatura_f}F");
    println!("Area del rectangulo: {area}");
}
```

**Rubrica**

- Distingues `f64` de `i32`.
- Sabes cuando un valor no necesita `mut`.
- Separaste la logica en funciones.
- Puedes explicar la inferencia de tipos del compilador.

## Dia 3. Tuplas y arrays

**Enunciado**

Dado un array de enteros, calcula suma, promedio, minimo y maximo.

**Solucion guia**

```rust
fn estadisticas(valores: [i32; 5]) -> (i32, f64, i32, i32) {
    let mut suma = 0;
    let mut minimo = valores[0];
    let mut maximo = valores[0];

    for valor in valores {
        suma += valor;
        if valor < minimo {
            minimo = valor;
        }
        if valor > maximo {
            maximo = valor;
        }
    }

    let promedio = suma as f64 / 5.0;
    (suma, promedio, minimo, maximo)
}

fn main() {
    let datos = [7, 3, 9, 1, 6];
    let (suma, promedio, minimo, maximo) = estadisticas(datos);

    println!("Suma: {suma}");
    println!("Promedio: {promedio}");
    println!("Minimo: {minimo}");
    println!("Maximo: {maximo}");
}
```

**Rubrica**

- Sabes usar arrays de tamano fijo.
- Sabes devolver varios valores con una tupla.
- Entiendes el destructuring de tuplas.
- Puedes recorrer un array con `for`.

## Dia 4. Funciones y expresiones

**Enunciado**

Implementa funciones `sumar`, `restar`, `es_par` y `maximo`.

**Solucion guia**

```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn restar(a: i32, b: i32) -> i32 {
    a - b
}

fn es_par(n: i32) -> bool {
    n % 2 == 0
}

fn maximo(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn main() {
    println!("Suma: {}", sumar(10, 5));
    println!("Resta: {}", restar(10, 5));
    println!("Es par 8: {}", es_par(8));
    println!("Maximo: {}", maximo(10, 5));
}
```

**Rubrica**

- Distingues parametros y tipo de retorno.
- Entiendes que la ultima expresion puede retornar sin `return`.
- Sabes cuando una funcion devuelve `bool`.
- El codigo es pequeno y legible.

## Dia 5. Control de flujo

**Enunciado**

Escribe un programa que calcule el factorial de un numero y ademas imprima su tabla de multiplicar del 1 al 10.

**Solucion guia**

```rust
fn factorial(n: u32) -> u32 {
    let mut resultado = 1;
    for i in 1..=n {
        resultado *= i;
    }
    resultado
}

fn imprimir_tabla(n: u32) {
    for i in 1..=10 {
        println!("{n} x {i} = {}", n * i);
    }
}

fn main() {
    let numero = 5;
    println!("Factorial de {numero}: {}", factorial(numero));
    imprimir_tabla(numero);
}
```

**Rubrica**

- Sabes usar `for` con rangos.
- Entiendes `1..=n`.
- Separas calculo e impresion.
- Puedes justificar por que no usaste `while`.

## Dia 6. Match e if let

**Enunciado**

Clasifica una nota numerica en `A`, `B`, `C`, `D` o `F`.

**Solucion guia**

```rust
fn clasificar_nota(nota: u32) -> &'static str {
    match nota {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Nota invalida",
    }
}

fn main() {
    let nota = 84;
    println!("Clasificacion: {}", clasificar_nota(nota));
}
```

**Rubrica**

- Usaste `match` de forma exhaustiva.
- Entiendes los rangos dentro de `match`.
- Sabes que `match` debe cubrir todos los casos.
- Puedes explicar cuando `if` seria menos claro.

## Dia 7. Mini proyecto: calculadora CLI

**Enunciado**

Crea una calculadora en consola que lea dos numeros y una operacion (`+`, `-`, `*`, `/`).

**Solucion guia**

```rust
use std::io;

fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

fn main() {
    println!("Primer numero:");
    let a: f64 = leer_linea().parse().unwrap();

    println!("Segundo numero:");
    let b: f64 = leer_linea().parse().unwrap();

    println!("Operacion (+, -, *, /):");
    let op = leer_linea();

    let resultado = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => {
            println!("Operacion invalida");
            return;
        }
    };

    println!("Resultado: {resultado}");
}
```

**Rubrica**

- El programa lee desde teclado.
- Convierte texto a numero con `parse`.
- Usa `match` para la operacion.
- Si la operacion no existe, sale con un mensaje claro.

## Dia 8. String vs &str

**Enunciado**

Escribe una funcion que reciba un nombre como `&str` y devuelva un saludo como `String`.

**Solucion guia**

```rust
fn construir_saludo(nombre: &str) -> String {
    format!("Hola, {nombre}!")
}

fn main() {
    let nombre = "Lucia";
    let saludo = construir_saludo(nombre);
    println!("{saludo}");
}
```

**Rubrica**

- Entiendes que `&str` es una referencia a texto.
- Entiendes que `String` es texto poseido y mutable.
- No forzaste `String` cuando solo necesitabas leer.
- Sabes por que `format!` devuelve `String`.

## Dia 9. Ownership

**Enunciado**

Demuestra con codigo la diferencia entre mover un `String` y clonarlo.

**Solucion guia**

```rust
fn main() {
    let original = String::from("Rust");
    let copia = original.clone();

    println!("Original: {original}");
    println!("Copia: {copia}");

    let movido = original;
    println!("Movido: {movido}");

    // println!("{original}"); // Ya no compila: original fue movido.
}
```

**Rubrica**

- Entiendes que `clone()` duplica datos.
- Entiendes que `let movido = original;` transfiere ownership.
- Sabes por que `String` no implementa `Copy`.
- Puedes identificar un uso innecesario de `clone()`.

## Dia 10. Referencias inmutables

**Enunciado**

Escribe una funcion que reciba una referencia a un `String` y muestre su longitud sin tomar ownership.

**Solucion guia**

```rust
fn imprimir_longitud(texto: &String) {
    println!("Texto: {texto}");
    println!("Longitud: {}", texto.len());
}

fn main() {
    let mensaje = String::from("aprendiendo rust");
    imprimir_longitud(&mensaje);
    println!("El valor sigue disponible: {mensaje}");
}
```

**Rubrica**

- Usaste `&mensaje` en la llamada.
- Entiendes que el valor sigue vivo despues de la funcion.
- Sabes que esta firma podria mejorarse a `&str`.
- No clonaste el `String`.

## Dia 11. Referencias mutables

**Enunciado**

Crea una funcion que agregue `" mundo"` a un `String` recibido por referencia mutable.

**Solucion guia**

```rust
fn agregar_mundo(texto: &mut String) {
    texto.push_str(" mundo");
}

fn main() {
    let mut saludo = String::from("hola");
    agregar_mundo(&mut saludo);
    println!("{saludo}");
}
```

**Rubrica**

- Declaraste la variable original como `mut`.
- Pasaste `&mut saludo`.
- Entiendes que solo puede haber una referencia mutable activa.
- Sabes diferenciar leer con `&` y modificar con `&mut`.

## Dia 12. Slices

**Enunciado**

Implementa una funcion `primera_palabra` que reciba `&str` y devuelva un slice con la primera palabra.

**Solucion guia**

```rust
fn primera_palabra(texto: &str) -> &str {
    for (i, byte) in texto.as_bytes().iter().enumerate() {
        if *byte == b' ' {
            return &texto[0..i];
        }
    }
    texto
}

fn main() {
    let frase = "rust es potente";
    let primera = primera_palabra(frase);
    println!("{primera}");
}
```

**Rubrica**

- La funcion devuelve una referencia, no un `String`.
- Entiendes que el resultado apunta al texto original.
- Sabes por que no conviene copiar si no hace falta.
- Puedes explicar que es un slice.

## Dia 13. Structs y metodos

**Enunciado**

Define una `struct Rectangulo` con un metodo `area` y otro `es_cuadrado`.

**Solucion guia**

```rust
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    fn es_cuadrado(&self) -> bool {
        self.ancho == self.alto
    }
}

fn main() {
    let rect = Rectangulo { ancho: 8, alto: 8 };
    println!("Area: {}", rect.area());
    println!("Es cuadrado: {}", rect.es_cuadrado());
}
```

**Rubrica**

- Separaste datos y comportamiento.
- Sabes cuando usar `&self`.
- Accedes a campos con notacion de punto.
- Entiendes la diferencia entre funcion asociada y metodo.

## Dia 14. Mini proyecto: gestor de tareas en memoria

**Enunciado**

Modela tareas con `id`, `titulo` y `completada`. Permite agregar y marcar una tarea como hecha.

**Solucion guia**

```rust
struct Task {
    id: u32,
    titulo: String,
    completada: bool,
}

fn marcar_completada(tareas: &mut Vec<Task>, id: u32) {
    for tarea in tareas {
        if tarea.id == id {
            tarea.completada = true;
        }
    }
}

fn main() {
    let mut tareas = vec![
        Task { id: 1, titulo: String::from("Estudiar Rust"), completada: false },
        Task { id: 2, titulo: String::from("Hacer ejercicio"), completada: false },
    ];

    marcar_completada(&mut tareas, 1);

    for tarea in &tareas {
        println!("{} - {} - {}", tarea.id, tarea.titulo, tarea.completada);
    }
}
```

**Rubrica**

- Usaste una `struct` clara.
- Sabes mutar elementos de un `Vec`.
- El programa cambia solo la tarea pedida.
- Tu modelo ya permite crecer.

## Dia 15. Enums para estados

**Enunciado**

Reemplaza el `bool` de tarea completada por un enum con varios estados.

**Solucion guia**

```rust
enum EstadoTarea {
    Pendiente,
    EnProgreso,
    Hecha,
}

struct Task {
    id: u32,
    titulo: String,
    estado: EstadoTarea,
}

fn main() {
    let tarea = Task {
        id: 1,
        titulo: String::from("Estudiar enums"),
        estado: EstadoTarea::EnProgreso,
    };

    match tarea.estado {
        EstadoTarea::Pendiente => println!("Pendiente"),
        EstadoTarea::EnProgreso => println!("En progreso"),
        EstadoTarea::Hecha => println!("Hecha"),
    }
}
```

**Rubrica**

- Ya no dependes de un solo `bool`.
- El dominio queda mejor representado.
- Sabes usar `match` sobre enums.
- Puedes agregar estados nuevos sin romper el modelo mental.

## Dia 16. Option

**Enunciado**

Escribe una funcion que busque una tarea por `id` y devuelva `Option<&Task>`.

**Solucion guia**

```rust
struct Task {
    id: u32,
    titulo: String,
}

fn buscar_tarea(tareas: &[Task], id: u32) -> Option<&Task> {
    for tarea in tareas {
        if tarea.id == id {
            return Some(tarea);
        }
    }
    None
}

fn main() {
    let tareas = vec![
        Task { id: 1, titulo: String::from("Leer") },
        Task { id: 2, titulo: String::from("Escribir") },
    ];

    match buscar_tarea(&tareas, 2) {
        Some(tarea) => println!("Encontrada: {}", tarea.titulo),
        None => println!("No existe"),
    }
}
```

**Rubrica**

- Usaste `Option` en vez de valores falsos o vacios.
- Entiendes `Some` y `None`.
- La referencia devuelta apunta a un dato valido.
- No usaste `unwrap()`.

## Dia 17. Result

**Enunciado**

Escribe una funcion que reciba texto y lo convierta a entero devolviendo `Result<i32, String>`.

**Solucion guia**

```rust
fn parsear_numero(texto: &str) -> Result<i32, String> {
    match texto.trim().parse::<i32>() {
        Ok(numero) => Ok(numero),
        Err(_) => Err(format!("No se pudo convertir '{texto}' a entero")),
    }
}

fn main() {
    match parsear_numero("42") {
        Ok(n) => println!("Numero: {n}"),
        Err(e) => println!("Error: {e}"),
    }
}
```

**Rubrica**

- Diferencias `Ok` y `Err`.
- Evitaste `panic!` para un error recuperable.
- El mensaje de error es util.
- La firma expresa claramente el posible fallo.

## Dia 18. Operador ?

**Enunciado**

Lee un archivo de texto y devuelve su contenido usando `Result` y `?`.

**Solucion guia**

```rust
use std::fs;
use std::io;

fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    Ok(contenido)
}

fn main() {
    match leer_archivo("datos.txt") {
        Ok(texto) => println!("{texto}"),
        Err(e) => println!("Error al leer archivo: {e}"),
    }
}
```

**Rubrica**

- Entiendes que `?` propaga el error.
- La funcion devuelve `Result`.
- No escribiste `match` innecesarios dentro de la funcion.
- Puedes explicar por que `main` decide que hacer con el error.

## Dia 19. Vec, String y HashMap

**Enunciado**

Crea un contador de palabras usando `HashMap<String, usize>`.

**Solucion guia**

```rust
use std::collections::HashMap;

fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    let mut conteos = HashMap::new();

    for palabra in texto.split_whitespace() {
        let contador = conteos.entry(palabra.to_string()).or_insert(0);
        *contador += 1;
    }

    conteos
}

fn main() {
    let texto = "rust es rapido y rust es seguro";
    let resultado = contar_palabras(texto);
    println!("{resultado:?}");
}
```

**Rubrica**

- Usaste `HashMap` correctamente.
- Entiendes `entry(...).or_insert(...)`.
- Conviertes `&str` a `String` solo donde hace falta.
- Tu solucion no usa indices manuales.

## Dia 20. Iteradores

**Enunciado**

Filtra los numeros pares de un vector, multiplcalos por 10 y reune el resultado en un nuevo vector.

**Solucion guia**

```rust
fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6];

    let procesados: Vec<i32> = numeros
        .iter()
        .filter(|n| **n % 2 == 0)
        .map(|n| n * 10)
        .collect();

    println!("{procesados:?}");
}
```

**Rubrica**

- Sabes encadenar `iter`, `filter`, `map`, `collect`.
- Entiendes por que `iter()` presta y no consume.
- Puedes reescribirlo con `for` y comparar estilos.
- Sabes leer cierres simples `|n|`.

## Dia 21. Mini proyecto: analizador de texto

**Enunciado**

Lee un texto y muestra numero de lineas, palabras y caracteres.

**Solucion guia**

```rust
fn analizar(texto: &str) -> (usize, usize, usize) {
    let lineas = texto.lines().count();
    let palabras = texto.split_whitespace().count();
    let caracteres = texto.chars().count();
    (lineas, palabras, caracteres)
}

fn main() {
    let texto = "Rust es seguro.\nRust es rapido.";
    let (lineas, palabras, caracteres) = analizar(texto);

    println!("Lineas: {lineas}");
    println!("Palabras: {palabras}");
    println!("Caracteres: {caracteres}");
}
```

**Rubrica**

- Usaste metodos de `&str` de forma idiomatica.
- Separaste la logica de analisis.
- Puedes ampliar la funcion sin rehacer todo.
- El programa ya parece una herramienta real.

## Dia 22. Modulos

**Enunciado**

Divide el codigo del analizador en modulos. Usa un modulo `stats` que exponga una funcion publica.

**Solucion guia**

```rust
mod stats {
    pub fn contar_palabras(texto: &str) -> usize {
        texto.split_whitespace().count()
    }
}

fn main() {
    let texto = "uno dos tres";
    let total = stats::contar_palabras(texto);
    println!("Palabras: {total}");
}
```

**Rubrica**

- Sabes declarar un modulo con `mod`.
- Sabes hacer publica una funcion con `pub`.
- Entiendes que la logica puede vivir fuera de `main`.
- Puedes imaginar la misma estructura en archivos separados.

## Dia 23. Tests

**Enunciado**

Escribe tests para una funcion `es_par` y una funcion `primera_palabra`.

**Solucion guia**

```rust
fn es_par(n: i32) -> bool {
    n % 2 == 0
}

fn primera_palabra(texto: &str) -> &str {
    match texto.split_whitespace().next() {
        Some(palabra) => palabra,
        None => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_es_par() {
        assert!(es_par(8));
        assert!(!es_par(5));
    }

    #[test]
    fn prueba_primera_palabra() {
        assert_eq!(primera_palabra("rust es genial"), "rust");
        assert_eq!(primera_palabra(""), "");
    }
}
```

**Rubrica**

- Sabes donde van los tests unitarios.
- Cubriste un caso normal y un caso borde.
- Puedes correr `cargo test`.
- Entiendes el valor de pruebas pequenas.

## Dia 24. Pattern matching avanzado

**Enunciado**

Modela comandos de usuario con un enum y procesa cada variante con `match`.

**Solucion guia**

```rust
enum Comando {
    Agregar(String),
    Completar(u32),
    Salir,
}

fn ejecutar(comando: Comando) {
    match comando {
        Comando::Agregar(texto) if !texto.is_empty() => {
            println!("Agregando tarea: {texto}");
        }
        Comando::Agregar(_) => println!("Titulo vacio"),
        Comando::Completar(id) => println!("Completando tarea {id}"),
        Comando::Salir => println!("Saliendo"),
    }
}

fn main() {
    ejecutar(Comando::Agregar(String::from("Repasar match")));
    ejecutar(Comando::Completar(3));
    ejecutar(Comando::Salir);
}
```

**Rubrica**

- Usaste enums con datos asociados.
- Incluiste un guard con `if`.
- Tu `match` es exhaustivo.
- Entiendes por que esto modela mejor que varios strings sueltos.

## Dia 25. Traits

**Enunciado**

Crea un trait `Resumen` y aplicalo a dos tipos distintos.

**Solucion guia**

```rust
trait Resumen {
    fn resumen(&self) -> String;
}

struct Libro {
    titulo: String,
}

struct Tarea {
    titulo: String,
}

impl Resumen for Libro {
    fn resumen(&self) -> String {
        format!("Libro: {}", self.titulo)
    }
}

impl Resumen for Tarea {
    fn resumen(&self) -> String {
        format!("Tarea: {}", self.titulo)
    }
}

fn imprimir_resumen<T: Resumen>(item: &T) {
    println!("{}", item.resumen());
}

fn main() {
    let libro = Libro { titulo: String::from("The Book") };
    let tarea = Tarea { titulo: String::from("Practicar traits") };

    imprimir_resumen(&libro);
    imprimir_resumen(&tarea);
}
```

**Rubrica**

- Separaste comportamiento comun con un trait.
- Implementaste el trait para dos structs.
- Usaste el trait en una funcion generica.
- Entiendes la idea de polimorfismo estatico.

## Dia 26. Genericos

**Enunciado**

Implementa una funcion generica `maximo` que funcione con cualquier tipo comparable.

**Solucion guia**

```rust
fn maximo<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", maximo(10, 7));
    println!("{}", maximo(3.5, 4.2));
}
```

**Rubrica**

- Entiendes que `T` representa un tipo generico.
- Sabes por que hace falta `PartialOrd`.
- Puedes explicar por que aqui usamos `Copy`.
- Sabes que un bound debe ser el minimo necesario.

## Dia 27. Lifetimes

**Enunciado**

Escribe la funcion `mas_largo` que recibe dos `&str` y devuelve el mas largo.

**Solucion guia**

```rust
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = "corto";
    let b = "mucho mas largo";
    println!("{}", mas_largo(a, b));
}
```

**Rubrica**

- Entiendes que el lifetime relaciona entradas y salida.
- Sabes que no esta alargando la vida real del dato.
- Puedes explicar la frase: "la salida vive como mucho lo que vivan las entradas".
- No intentaste devolver referencias a temporales.

## Dia 28. Mini proyecto: libreria reutilizable

**Enunciado**

Extrae la logica de tareas a una API reutilizable con una `struct TaskManager`.

**Solucion guia**

```rust
struct Task {
    id: u32,
    titulo: String,
}

struct TaskManager {
    tareas: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        Self { tareas: Vec::new() }
    }

    fn agregar(&mut self, id: u32, titulo: &str) {
        self.tareas.push(Task {
            id,
            titulo: titulo.to_string(),
        });
    }

    fn total(&self) -> usize {
        self.tareas.len()
    }
}

fn main() {
    let mut manager = TaskManager::new();
    manager.agregar(1, "Refactorizar");
    manager.agregar(2, "Probar");
    println!("Total: {}", manager.total());
}
```

**Rubrica**

- Encapsulaste el `Vec<Task>` dentro de otra struct.
- El API es mas comodo que manipular el vector directo.
- La logica central ya podria moverse a `lib.rs`.
- Tu codigo esta listo para crecer.

## Dia 29. Concurrencia

**Enunciado**

Crea dos hilos que impriman mensajes y espera a que ambos terminen.

**Solucion guia**

```rust
use std::thread;

fn main() {
    let h1 = thread::spawn(|| {
        for i in 1..=3 {
            println!("Hilo 1: {i}");
        }
    });

    let h2 = thread::spawn(|| {
        for i in 1..=3 {
            println!("Hilo 2: {i}");
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
```

**Rubrica**

- Sabes crear hilos con `thread::spawn`.
- Sabes esperar con `join`.
- Entiendes que el orden de impresion puede variar.
- Ya tienes una base para estudiar `Arc`, `Mutex` y canales.

## Dia 30. Proyecto final guiado

**Enunciado**

Disena una app CLI de tareas con persistencia basica. En este ultimo dia, implementa un esqueleto con comandos `add` y `list`.

**Solucion guia**

```rust
use std::fs;

#[derive(Debug)]
struct Task {
    titulo: String,
}

fn cargar_tareas(ruta: &str) -> Vec<Task> {
    match fs::read_to_string(ruta) {
        Ok(contenido) => contenido
            .lines()
            .map(|linea| Task {
                titulo: linea.to_string(),
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn guardar_tareas(ruta: &str, tareas: &[Task]) {
    let contenido = tareas
        .iter()
        .map(|t| t.titulo.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(ruta, contenido).unwrap();
}

fn main() {
    let ruta = "tareas.txt";
    let mut tareas = cargar_tareas(ruta);

    tareas.push(Task {
        titulo: String::from("Cerrar curso de Rust"),
    });

    guardar_tareas(ruta, &tareas);

    for tarea in &tareas {
        println!("{:?}", tarea);
    }
}
```

**Rubrica**

- El programa carga y guarda datos simples.
- Modelaste tareas con una struct.
- Ya juntas I/O, colecciones y organizacion basica.
- Sabes como extenderlo con `done`, `delete` y `search`.

## Como corregirte bien

En cada dia, intenta responder estas preguntas:

- Mi solucion compila sin errores.
- Puedo explicar por que cada referencia es `&` o `&mut`.
- Use `String`, `&str`, `Vec`, `Option` o `Result` por una razon concreta.
- Podria dividir mi codigo en funciones mas pequenas.
- Hay un `clone()` que podria evitar.

## Errores tipicos que debes aprender a diagnosticar

- `borrow of moved value`: moviste un valor y luego lo quisiste reutilizar.
- `cannot borrow as mutable because it is also borrowed as immutable`: mezclaste prestamos incompatibles.
- `expected &str, found String`: la firma y el argumento no coinciden.
- `cannot return reference to local variable`: estas devolviendo una referencia a un valor que va a morir.
- `mismatched types`: una rama o retorno no coincide con el tipo esperado.
- `trait bound not satisfied`: te falta implementar o declarar un trait requerido.

## Siguiente paso recomendado

Cuando cierres este manual, continua con estas crates:

- `serde`
- `clap`
- `anyhow`
- `thiserror`
- `reqwest`
- `tokio`
- `axum`

Y sobre todo: reescribe tus mini proyectos. En Rust, repetir con intencion acelera mucho mas que leer teoria nueva.
