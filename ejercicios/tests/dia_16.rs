use rust_practica::dia_16::buscar_valor;

#[test]
fn vacio() {
    assert_eq!(buscar_valor(&[], 0), None);
}

#[test]
fn presente() {
    assert_eq!(buscar_valor(&[4, -2, 7], 1), Some(-2));
}

#[test]
fn fuera_de_rango() {
    assert_eq!(buscar_valor(&[1], usize::MAX), None);
}
