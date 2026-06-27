# Preguntas de Repaso y Respuestas

Este documento te sirve para comprobar si de verdad estas entendiendo el curso y no solo copiando ejemplos.

## Semana 1

### Pregunta

Que diferencia hay entre compilar y ejecutar?

### Respuesta

Compilar convierte tu codigo Rust en un ejecutable. Ejecutar corre ese binario ya compilado.

### Pregunta

Que diferencia hay entre una variable mutable y una inmutable?

### Respuesta

La inmutable no puede reasignarse despues de crearse; la mutable si, usando `mut`.

### Pregunta

Por que Rust distingue entre expresiones y sentencias?

### Respuesta

Porque muchas construcciones producen valores y eso afecta al estilo del lenguaje y al retorno de funciones y bloques.

### Pregunta

Cuando preferirias `match` frente a `if`?

### Respuesta

Cuando el dominio tiene casos cerrados y quieres cobertura exhaustiva o trabajar con patrones.

## Semana 2

### Pregunta

Que significa que un `String` se mueve?

### Respuesta

Que la propiedad del valor se transfiere y la variable original ya no puede usarse.

### Pregunta

Cuando deberia una funcion recibir `&str` en vez de `String`?

### Respuesta

Cuando solo necesita leer texto y no poseerlo ni modificarlo.

### Pregunta

Por que Rust permite muchas referencias inmutables pero solo una mutable?

### Respuesta

Para evitar que existan lecturas y escrituras peligrosas sobre el mismo dato al mismo tiempo.

### Pregunta

Que es un slice?

### Respuesta

Una vista prestada sobre parte de una coleccion, sin copiar sus datos.

## Semana 3

### Pregunta

Para que sirve `Option<T>`?

### Respuesta

Para representar presencia o ausencia de valor sin usar `null`.

### Pregunta

Para que sirve `Result<T, E>`?

### Respuesta

Para representar operaciones que pueden tener exito o error recuperable.

### Pregunta

Que hace el operador `?`?

### Respuesta

Si el valor es `Ok`, extrae su contenido; si es `Err`, retorna temprano el error desde la funcion.

### Pregunta

Cuando elegirias `HashMap` sobre `Vec`?

### Respuesta

Cuando necesites buscar por clave en lugar de recorrer secuencialmente por posicion.

## Semana 4

### Pregunta

Que papel cumplen los modulos?

### Respuesta

Organizan el codigo y controlan su visibilidad.

### Pregunta

Que diferencia hay entre un struct y un enum?

### Respuesta

Un struct representa una sola forma fija de datos; un enum representa varias variantes posibles.

### Pregunta

Que es un trait?

### Respuesta

Un contrato que expresa comportamiento compartido entre tipos.

### Pregunta

Que problema resuelven los genericos?

### Respuesta

Permiten reutilizar logica para varios tipos sin perder seguridad de tipos.

### Pregunta

Que expresan los lifetimes?

### Respuesta

Relaciones entre referencias para que el compilador compruebe que siguen siendo validas.

## Semana 5

### Pregunta

Que ventaja tiene `thread::spawn` junto con el sistema de tipos de Rust?

### Respuesta

Que la concurrencia se construye con chequeos fuertes sobre ownership y datos compartidos.

### Pregunta

Por que conviene dejar `async` para despues de ownership y `Result`?

### Respuesta

Porque `async` se apoya en conceptos previos del lenguaje y se vuelve mucho mas facil si ya entiendes el modelo base.

## Prueba corta final

Intenta responder estas diez sin mirar:

1. Que diferencia hay entre `String` y `&str`?
2. Que significa mover un valor?
3. Que representa `None`?
4. Para que sirve `Err`?
5. Cuando usarias un enum?
6. Que hace `collect()`?
7. Que hace `pub`?
8. Que ventaja tiene un trait?
9. Por que un `clone()` repetido puede ser una mala senal?
10. Cuando devolverias `Result` y cuando `Option`?

Si puedes responderlas bien, vas muy encaminado.
