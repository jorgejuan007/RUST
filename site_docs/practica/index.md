# Ejercicios con corrección automática

Este paquete independiente contiene doce ejercicios para los días 9–12, 16–20 y 25–27.
Las plantillas compilan, pero sus tests fallan hasta que implementes los `todo!()`.
Las soluciones están separadas y nunca se copian sobre tu trabajo.

Desde la raíz del repositorio:

```bash
python3 scripts/practica.py listar
python3 scripts/practica.py pista 9
python3 scripts/practica.py pista 9 --nivel 2
python3 scripts/practica.py comprobar 9
```

1. Lee el enunciado y edita `ejercicios/src/dia_09.rs`.
2. Conserva la firma pública y completa la función.
3. Ejecuta `comprobar 9`; el código de salida es cero solo cuando pasan las pruebas.
4. Usa las pistas de los niveles 1, 2 y 3 si necesitas ayuda.
5. Consulta la solución después de intentarlo: `python3 scripts/practica.py solucion 9`.

| Día | Concepto | Qué comprueban los tests |
|---|---|---|
| 9 | Ownership | Devolver el texto y contar caracteres sin perder el valor |
| 10 | Préstamo inmutable | Aceptar `&str` y conservar el String del llamador |
| 11 | Préstamo mutable | Modificar el texto, whitespace y mayúsculas Unicode |
| 12 | Slices | Primera palabra como préstamo, sin cortar UTF-8 |
| 16 | Option | Valores presentes, slices vacíos e índices fuera de rango |
| 17 | Result | Enteros positivos, entradas inválidas y desbordamientos |
| 18 | Operador `?` | Errores con línea y sumas comprobadas |
| 19 | HashMap | Frecuencias normalizadas y palabras Unicode |
| 20 | Iteradores | Pares negativos y acumulación en `i64` |
| 25 | Traits | Contrato compartido y tipos adicionales definidos por el alumno |
| 26 | Genéricos | Valores sin Copy/Clone, slices vacíos y primer empate |
| 27 | Lifetimes | Unicode y préstamos que siguen ligados al texto original |

Para formatear tu trabajo:

```bash
cargo fmt --manifest-path ejercicios/Cargo.toml
```

La automatización valida las soluciones en una copia temporal:

```bash
python3 scripts/practica.py verificar-soluciones
```

El paquete no tiene dependencias externas. El curso principal puede seguir pasando
sus pruebas aunque todavía tengas ejercicios pendientes. Las soluciones de referencia
demuestran una forma de resolverlos; los tests aceptan cualquier implementación que
cumpla el contrato. Para ownership y borrowing, explica también por qué las firmas
permiten o impiden mover o modificar el dato.

Consulta el [manual del curso](../curso/manual_30_dias.md) y la
[guía de práctica](../complementos/27_practica_con_correccion.md).
