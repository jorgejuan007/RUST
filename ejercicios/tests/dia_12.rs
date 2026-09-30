use rust_practica::dia_12::primera_palabra;

#[test]
fn vacio() {
    assert_eq!(primera_palabra(" \t\n"), "");
}

#[test]
fn espacios() {
    assert_eq!(primera_palabra(" \tRust es claro"), "Rust");
}

#[test]
fn unicode_y_slice() {
    let texto = String::from("árbol verde");
    let palabra = primera_palabra(&texto);
    assert_eq!(palabra, "árbol");
    assert_eq!(palabra.as_ptr(), texto.as_ptr());
}
