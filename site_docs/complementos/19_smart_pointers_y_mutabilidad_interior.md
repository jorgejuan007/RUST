# Smart Pointers y Mutabilidad Interior

Este es uno de los saltos mas importantes entre Rust basico y Rust intermedio.

## Cuando aparecen

Mientras el ownership lineal te baste, todo va bastante directo.

Los smart pointers aparecen cuando necesitas:

- recursividad
- ownership compartido
- mutacion compartida controlada
- sincronizacion entre hilos

## `Box<T>`

`Box<T>` sirve sobre todo para:

- mover datos al heap
- representar tipos recursivos
- reducir tamano de un enum recursivo

Ejemplo tipico:

- un arbol
- una expresion matematica
- una lista enlazada didactica

## `Rc<T>`

`Rc<T>` da ownership compartido en un solo hilo.

Conviene cuando varias partes de tu programa necesitan leer el mismo valor y su vida util no encaja bien con prestamos normales.

No vale para multihilo.

## `RefCell<T>`

`RefCell<T>` permite mutabilidad interior con comprobacion en runtime.

Traduccion practica:

- el compilador deja pasar el patron
- si rompes las reglas de prestamo, fallas en ejecucion

Por eso es util, pero tambien exige disciplina.

## `Cell<T>`

`Cell<T>` es buena para valores `Copy` pequenos:

- contadores
- flags
- caches diminutas

Sin referencias prestadas largas y sin el coste mental de `RefCell`.

## `Arc<T>`

`Arc<T>` es la version compartida segura para varios hilos.

Suele combinarse con:

- `Mutex<T>`
- `RwLock<T>`

## `Mutex<T>` y `RwLock<T>`

`Mutex<T>`:

- un escritor a la vez
- modelo simple
- muy comun

`RwLock<T>`:

- muchos lectores o un escritor
- mas util cuando lees mucho y escribes poco

## Regla de eleccion rapida

- recursividad: `Box<T>`
- compartir en un hilo: `Rc<T>`
- compartir y mutar en un hilo: `Rc<RefCell<T>>`
- compartir entre hilos: `Arc<T>`
- compartir y mutar entre hilos: `Arc<Mutex<T>>` o `Arc<RwLock<T>>`

## Ejemplo ejecutable en este repo

Archivo:

- [src/bin/bonus_smart_pointers.rs](../referencia/codigo/src/bin/bonus_smart_pointers.rs)

Ejecuta:

```bash
cargo run --bin bonus_smart_pointers
```

Ese bonus ensena cuatro piezas utiles:

- `Box` para una expresion recursiva
- `Rc<RefCell<_>>` para estado compartido en un hilo
- `Cell` para un contador `Copy`
- `Arc<Mutex<_>>` para coordinacion entre hilos

## Ojo con esta trampa

Si tu primera reaccion es envolver todo en `Rc<RefCell<_>>`, seguramente aun no has simplificado suficiente el modelado.

Primero intenta:

- ownership claro
- prestamos
- funciones mas pequenas
- estados mejor definidos

## Antipatrones comunes

- usar `Rc` en codigo multihilo
- esconder un mal diseno detras de `RefCell`
- bloquear un `Mutex` mas tiempo del necesario
- mezclar demasiadas responsabilidades dentro del valor protegido

## Senal de diseno sano

Un smart pointer deberia aparecer porque el problema lo pide, no porque "asi compila".

## Cierre

Entender estas herramientas te permite modelar problemas mas reales sin pelearte tanto con ownership.
