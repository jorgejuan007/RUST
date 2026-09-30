# Arquitectura y Refactor en Rust

Una de las mejores cosas que puedes aprender despues de la sintaxis es a organizar proyectos sin dejar que crezcan desordenados.

## Regla general

Separa:

- interfaz
- logica
- almacenamiento
- errores

## Estructura sana para proyectos pequenos

```text
src/
|-- main.rs
|-- lib.rs
|-- models.rs
|-- storage.rs
`-- errors.rs
```

## Interfaz vs logica

`main.rs` o un binario en `src/bin/` deberia:

- leer argumentos
- llamar funciones
- imprimir resultados

No deberia contener toda la logica del negocio.

## Libreria interna

Mover logica a `lib.rs` o modulos internos tiene ventajas:

- puedes testear mejor
- reduces complejidad en `main`
- reaprovechas funciones
- diseñas APIs mas limpias

## Senales de que necesitas refactor

- `main.rs` crece demasiado
- una funcion hace tres o cuatro cosas
- repites el mismo parseo o validacion
- cuesta escribir tests
- aparecen muchos `if` y `match` desordenados

## Refactor recomendado en cuatro pasos

1. Extrae funciones pequenas.
2. Agrupa por responsabilidad.
3. Crea modulos.
4. Mueve la logica estable a la libreria.

## Buen patron

- binario = capa de entrada
- libreria = logica reutilizable
- tests = prueba del comportamiento

## Mal patron

- todo en `main`
- parseo, I/O, logica y formato mezclados
- errores tratados con `unwrap()` por todas partes

## Como aplicar esto a este repo

Ya tienes ejemplos de tres niveles:

- binarios didacticos cortos
- gestor std simple
- gestor JSON con `clap` y `serde`

Y ahora tambien una libreria reutilizable en:

- [src/lib.rs](https://github.com/jorgejuan007/RUST/blob/main/src/lib.rs)

## Meta real del refactor

Refactorizar no es embellecer. Es hacer el codigo:

- mas entendible
- mas testeable
- mas reutilizable
- menos fragil
