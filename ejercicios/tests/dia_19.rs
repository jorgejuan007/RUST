use rust_practica::dia_19::frecuencias;

#[test]
fn vacio() {
    assert!(frecuencias(" \n").is_empty());
}

#[test]
fn conteos() {
    let mapa = frecuencias("Rust rust RUST claro");
    assert_eq!(mapa.get("rust"), Some(&3));
    assert_eq!(mapa.get("claro"), Some(&1));
}

#[test]
fn unicode() {
    let mapa = frecuencias("ÁRBOL\tárbol 🦀");
    assert_eq!(mapa.get("árbol"), Some(&2));
    assert_eq!(mapa.get("🦀"), Some(&1));
}
