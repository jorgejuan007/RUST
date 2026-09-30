use rust_practica::dia_11::normalizar;

#[test]
fn vacio() {
    let mut texto = String::from(" \t\n");
    normalizar(&mut texto);
    assert_eq!(texto, "");
}

#[test]
fn espacios() {
    let mut texto = String::from("  RUST\tES\nCLARO  ");
    normalizar(&mut texto);
    assert_eq!(texto, "rust es claro");
}

#[test]
fn unicode() {
    let mut texto = String::from(" ÁRBOL 🦀 ");
    normalizar(&mut texto);
    assert_eq!(texto, "árbol 🦀");
}
