pub fn buscar_valor(valores: &[i32], posicion: usize) -> Option<i32> {
    valores.get(posicion).copied()
}
