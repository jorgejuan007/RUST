pub fn sumar_lineas(texto: &str) -> Result<i32, String> {
    let mut suma = 0_i32;
    for (indice, linea) in texto.lines().enumerate() {
        let linea = linea.trim();
        if linea.is_empty() {
            continue;
        }
        let valor = linea
            .parse::<i32>()
            .map_err(|_| format!("línea {}: entero inválido", indice + 1))?;
        suma = suma
            .checked_add(valor)
            .ok_or_else(|| format!("línea {}: desbordamiento", indice + 1))?;
    }
    Ok(suma)
}
