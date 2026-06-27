use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::Path;

fn parsear_numeros(contenido: &str) -> Result<Vec<i64>> {
    let mut numeros = Vec::new();

    for (indice, linea) in contenido.lines().enumerate() {
        let linea = linea.trim();

        if linea.is_empty() || linea.starts_with('#') {
            continue;
        }

        let numero = linea
            .parse::<i64>()
            .with_context(|| format!("linea {} invalida: {linea}", indice + 1))?;
        numeros.push(numero);
    }

    if numeros.is_empty() {
        bail!("el archivo no contiene numeros aprovechables");
    }

    Ok(numeros)
}

fn cargar_numeros(ruta: &Path) -> Result<Vec<i64>> {
    let contenido = fs::read_to_string(ruta)
        .with_context(|| format!("no se pudo leer el archivo {}", ruta.display()))?;

    parsear_numeros(&contenido)
        .with_context(|| format!("fallo al interpretar el contenido de {}", ruta.display()))
}

fn resumen(numeros: &[i64]) -> (usize, i64, f64) {
    let cantidad = numeros.len();
    let suma: i64 = numeros.iter().sum();
    let promedio = suma as f64 / cantidad as f64;
    (cantidad, suma, promedio)
}

fn main() -> Result<()> {
    let ruta = env::args()
        .nth(1)
        .unwrap_or_else(|| "data/bonus_numeros.txt".to_string());
    let ruta = Path::new(&ruta);

    let numeros = cargar_numeros(ruta)?;
    let (cantidad, suma, promedio) = resumen(&numeros);

    println!("Archivo: {}", ruta.display());
    println!("Valores: {:?}", numeros);
    println!("Cantidad: {cantidad}");
    println!("Suma: {suma}");
    println!("Promedio: {:.2}", promedio);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parsear_numeros, resumen};

    #[test]
    fn parsea_numeros_con_comentarios_y_lineas_vacias() {
        let numeros = parsear_numeros(
            "
            # valores de ejemplo
            10

            25
            40
            ",
        )
        .unwrap();

        assert_eq!(numeros, vec![10, 25, 40]);
    }

    #[test]
    fn detecta_linea_invalida() {
        let error = parsear_numeros("10\nhola\n20").unwrap_err();
        let mensaje = error.to_string();

        assert!(mensaje.contains("linea 2 invalida"));
    }

    #[test]
    fn calcula_resumen() {
        let (cantidad, suma, promedio) = resumen(&[10, 20, 30]);

        assert_eq!(cantidad, 3);
        assert_eq!(suma, 60);
        assert!((promedio - 20.0).abs() < f64::EPSILON);
    }
}
