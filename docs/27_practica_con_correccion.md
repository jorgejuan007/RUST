# Práctica con corrección automática

La práctica complementa el manual y las soluciones ejecutables. Aquí escribes tu
respuesta y recibes resultados de pruebas antes de ver la solución.

Consulta el [catálogo y las instrucciones](../ejercicios/README.md).

## Ruta recomendada

1. Completa los días 9, 10, 11 y 12 después del bloque de ownership y borrowing.
2. Completa los días 16 y 17 para practicar ausencia y errores recuperables.
3. Resuelve el día 18 sin `unwrap()` y propaga errores con `?`.
4. Cierra con los días 19 y 20: colecciones e iteradores.

```bash
python3 scripts/practica.py comprobar 12
python3 scripts/practica.py pista 12 --nivel 2
```

Los tests incluyen entradas vacías, Unicode y casos límite. Una respuesta que funciona
solo con el ejemplo del enunciado puede fallar. Cuando pase, explica qué casos cubre y
qué restricciones expresa la firma. No cambies los tests para aprobar.

## Corrección y soluciones

La corrección usa `cargo test` sobre el paquete independiente `ejercicios/`.
Un fallo indica trabajo pendiente; no es un error de instalación si muestra el mensaje
del `todo!()`. El comando conserva tus archivos y devuelve un código de salida distinto
de cero cuando hay errores de compilación o pruebas fallidas.

`verificar-soluciones` comprueba todos los modelos de respuesta en una carpeta temporal.
Este comando forma parte del CI y no resuelve automáticamente tus ejercicios.
