use rust_practica::dia_20::suma_pares;

#[test]
fn vacio() {
    assert_eq!(suma_pares(&[]), 0);
}

#[test]
fn negativos() {
    assert_eq!(suma_pares(&[-4, -3, 0, 2, 5]), -2);
}

#[test]
fn acumulacion_grande() {
    assert_eq!(suma_pares(&[2147483646, 2147483646, i32::MIN]), 2147483644);
}
