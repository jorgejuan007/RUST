use rust_practica::dia_17::parsear_positivo;

#[test]
fn validos() {
    assert_eq!(parsear_positivo(" 42 "), Ok(42));
    assert_eq!(parsear_positivo("4294967295"), Ok(u32::MAX));
}

#[test]
fn invalidos() {
    for texto in ["", "hola", "0", "-1"] {
        assert!(parsear_positivo(texto).is_err(), "{texto}");
    }
}

#[test]
fn desbordamiento() {
    assert!(parsear_positivo("4294967296").is_err());
}
