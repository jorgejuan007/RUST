use rust_practica::dia_18::sumar_lineas;

#[test]
fn validos() {
    assert_eq!(sumar_lineas(" \n 2\n-1\n 4 \n"), Ok(5));
    assert_eq!(sumar_lineas(""), Ok(0));
}

#[test]
fn linea_invalida() {
    let error = sumar_lineas("1\nhola\n2").unwrap_err();
    assert!(error.contains("2"));
}

#[test]
fn desbordamientos() {
    assert!(sumar_lineas("2147483647\n1").is_err());
    assert!(sumar_lineas("-2147483648\n-1").is_err());
}
