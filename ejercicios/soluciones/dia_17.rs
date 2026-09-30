pub fn parsear_positivo(texto: &str) -> Result<u32, String> {
    let valor = texto
        .trim()
        .parse::<u32>()
        .map_err(|_| "entero inválido".to_owned())?;
    if valor == 0 {
        return Err("debe ser positivo".into());
    }
    Ok(valor)
}
