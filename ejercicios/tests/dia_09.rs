use rust_practica::dia_09::devolver_texto;

#[test]
fn vacio() {
    assert_eq!(devolver_texto(String::new()), (String::new(), 0));
}

#[test]
fn ascii() {
    assert_eq!(devolver_texto("Rust".into()), ("Rust".into(), 4));
}

#[test]
fn unicode() {
    assert_eq!(devolver_texto("Árbol 🦀".into()), ("Árbol 🦀".into(), 7));
}
