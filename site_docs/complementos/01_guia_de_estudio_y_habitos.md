# Guia de Estudio y Habitos

Este curso funciona mejor si lo estudias como practica deliberada y no como lectura pasiva. Rust recompensa mucho la repeticion consciente.

## Como estudiar cada sesion

Una sesion de 60 a 120 minutos puede dividirse asi:

1. Lectura rapida del tema: 10 a 20 minutos.
2. Escritura del ejemplo desde cero: 20 a 30 minutos.
3. Ejercicio del dia: 20 a 40 minutos.
4. Refactor y revision: 10 a 20 minutos.

## Regla de oro

No avances de tema solo porque "ya compila". Avanza cuando puedas hacer estas cuatro cosas:

- explicar con tus palabras que hace el codigo
- reescribirlo sin mirar
- modificarlo sin romperte
- justificar por que una firma usa `String`, `&str`, `Vec`, `Option` o `Result`

## Metodo recomendado

### Primera pasada

- Ejecuta el binario del dia.
- Lee el enunciado en el manual.
- Intenta resolverlo sin mirar la solucion.

### Segunda pasada

- Compara tu solucion con la guia.
- Marca diferencias de estilo, no solo de resultado.
- Reescribe la version final desde cero.

### Tercera pasada

- Cambia nombres, tipos o casos de prueba.
- Intenta introducir un error y entender el mensaje del compilador.
- Haz una micro nota de lo aprendido.

## Plantilla de notas diarias

Usa una libreta o un archivo por dia con este formato:

```text
Tema:
Lo que entendi:
Lo que aun me confunde:
Error del compilador que aparecio:
Como lo resolvi:
Que cambiaria en mi codigo:
```

## Checklist diario

- Ejecute el binario del dia.
- Implemente una solucion propia.
- Lei la solucion guia con calma.
- Refactorice una funcion o firma.
- Corri `cargo check`.
- Si habia tests, corri `cargo test`.

## Cuando te atasques

Haz esta secuencia antes de buscar mas teoria:

1. Reduce el problema al ejemplo mas pequeno posible.
2. Pregunta: "quien es el duenio del valor?".
3. Pregunta: "estoy leyendo o modificando?".
4. Revisa si de verdad necesitas poseer el dato.
5. Intenta cambiar `String` por `&str`, o `Vec<T>` por `&[T]`, si solo lees.

## Como practicar ownership sin frustrarte

Ownership se aprende mejor cuando comparas tres versiones del mismo programa:

- version que mueve
- version que presta
- version que clona

Despues preguntate:

- cual es la mas clara
- cual hace menos trabajo
- cual expresa mejor la intencion

## Como leer el compilador

No leas solo la ultima linea. En Rust suele haber tres pistas:

- donde se detecto el problema
- donde se origino el prestamo, movimiento o tipo
- que alternativa concreta sugiere el compilador

Muchas veces el error real no esta en la linea marcada, sino unas lineas antes.

## Buenas preguntas que debes hacerte siempre

- Esta funcion necesita poseer o solo leer?
- Esta mutacion de verdad es necesaria?
- Este `clone()` es una decision o un parche?
- Estoy modelando estados con un `enum` o con bools dispersos?
- Estoy devolviendo `Result` donde podria fallar?

## Rutina semanal recomendada

### Lunes a viernes

- Un dia por tema
- 60 a 90 minutos por sesion

### Sabado

- Repite dos dias anteriores sin mirar
- Refactoriza un mini proyecto
- Resuelve un reto extra

### Domingo

- Descanso o lectura ligera
- Revisa errores del compilador que ya sabes interpretar

## Como consolidar al final de cada semana

Semana 1:

- deberias poder escribir funciones simples
- deberias sentirte comodo con `if`, `for`, `match`

Semana 2:

- deberias poder explicar ownership y borrowing
- deberias distinguir `String` y `&str`

Semana 3:

- deberias usar `Option`, `Result`, `Vec` y `HashMap`
- deberias leer errores simples sin bloquearte

Semana 4:

- deberias separar codigo en modulos
- deberias escribir tests y traits simples

Semana 5:

- deberias entender el mapa general del lenguaje
- deberias poder iniciar un proyecto propio

## Regla de progreso real

Tu objetivo no es memorizar todo Rust. Tu objetivo es llegar a pensar asi:

- "este dato lo puedo prestar"
- "aqui necesito un enum"
- "esto deberia devolver Result"
- "este error del compilador tiene sentido"

Cuando empieces a pensar de ese modo, ya estaras programando en Rust y no solo leyendo Rust.
