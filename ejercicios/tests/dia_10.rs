use rust_practica::dia_10::longitud;

#[test]
fn vacio() {
    assert_eq!(longitud(""), 0);
}

#[test]
fn conserva_ownership() {
    let texto = String::from("Rust");
    assert_eq!(longitud(&texto), 4);
    assert_eq!(texto, "Rust");
}

#[test]
fn unicode() {
    assert_eq!(longitud("é🦀"), 2);
}
