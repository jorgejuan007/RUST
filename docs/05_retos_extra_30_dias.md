# Retos Extra para los 30 Dias

Estos retos estan pensados para hacer el curso mas profundo. No incluyen solucion completa a proposito: la idea es que practiques con apoyo del manual y de los binarios.

## Semana 1

### Dia 1

Reto: imprime tambien una resta, una multiplicacion y un promedio de dos numeros.

Pistas:

- crea variables separadas
- usa `println!` mas de una vez

### Dia 2

Reto: agrega conversion de Fahrenheit a Celsius y calcula el area de un triangulo.

Pistas:

- reutiliza el patron de funciones puras

### Dia 3

Reto: devuelve tambien la mediana de un array pequeno ya ordenado.

Pistas:

- piensa en el indice central

### Dia 4

Reto: crea una funcion `minimo` y otra `signo` que diga si un numero es positivo, negativo o cero.

### Dia 5

Reto: imprime los primeros `n` numeros de Fibonacci.

Pistas:

- usa variables mutables para ir actualizando el estado

### Dia 6

Reto: clasifica un caracter como vocal, consonante, numero u otro.

### Dia 7

Reto: evita `unwrap()` en la calculadora CLI y muestra errores amigables si el usuario escribe mal.

## Semana 2

### Dia 8

Reto: construye una funcion que reciba nombre y apellido como `&str` y devuelva un nombre completo con formato.

### Dia 9

Reto: reescribe un ejemplo con `clone()`, luego intenta resolverlo solo con prestamos.

### Dia 10

Reto: cambia firmas que usan `&String` por `&str` y explica por que mejoran.

### Dia 11

Reto: modifica una lista de enteros sumando 1 a cada elemento con una referencia mutable.

### Dia 12

Reto: devuelve la ultima palabra de una frase como slice.

### Dia 13

Reto: agrega a `Rectangulo` un metodo `puede_contener(&self, otro: &Rectangulo) -> bool`.

### Dia 14

Reto: en el gestor de tareas, agrega una funcion que cuente cuantas tareas siguen pendientes.

## Semana 3

### Dia 15

Reto: agrega al enum de estado una variante `Bloqueada(String)` con motivo.

### Dia 16

Reto: implementa `buscar_tarea_mut` que devuelva `Option<&mut Task>`.

### Dia 17

Reto: devuelve un error distinto si la cadena esta vacia y otro si contiene texto no numerico.

### Dia 18

Reto: lee un archivo y cuenta cuantas lineas no vacias tiene, devolviendo `Result`.

### Dia 19

Reto: normaliza el texto a minusculas antes de contar palabras.

### Dia 20

Reto: usa `fold` para sumar una lista y para encontrar el maximo.

### Dia 21

Reto: haz que el analizador muestre tambien la palabra mas frecuente.

## Semana 4

### Dia 22

Reto: separa un modulo `parser` y otro `stats` en archivos distintos.

### Dia 23

Reto: escribe un test que verifique que una cadena vacia produce cero palabras.

### Dia 24

Reto: parsea comandos reales desde `std::env::args()`.

### Dia 25

Reto: crea un trait `Identificable` con un metodo `id(&self) -> u32`.

### Dia 26

Reto: escribe una funcion generica que compare dos slices y diga si tienen la misma longitud.

### Dia 27

Reto: crea una struct que contenga `&str` y practica que lifetimes necesita.

### Dia 28

Reto: agrega al `TaskManager` una busqueda por `id`.

## Semana 5

### Dia 29

Reto: usa un canal para que un hilo envie cinco mensajes y el hilo principal los reciba.

### Dia 30

Reto: convierte tu proyecto final en una CLI que entienda `add`, `list`, `done` y `delete`.

## Criterio de exito para los retos

Un reto esta bien resuelto si:

- compila
- la solucion es clara
- no usa `clone()` por reflejo
- modela bien el problema
- puedes explicar por que funciona

## Consejo final

Si un reto te sale muy rapido, complicalo:

- agrega validaciones
- devuelve `Result`
- escribe tests
- separa en modulos
