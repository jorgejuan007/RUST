pub fn maximo<T: Ord>(valores: &[T]) -> Option<&T> {
    let mut mayor = valores.first()?;
    for valor in &valores[1..] {
        if valor > mayor {
            mayor = valor;
        }
    }
    Some(mayor)
}
