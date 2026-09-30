use rust_practica::dia_27::{mas_largo, Extracto};

#[test]
fn longitud_unicode_y_entradas_vacias() {
    assert_eq!(mas_largo("🌳🌳", "abc"), "abc");
    assert_eq!(mas_largo("", "texto"), "texto");
    assert_eq!(mas_largo("", ""), "");
}

#[test]
fn devuelve_la_entrada_correcta_y_primer_empate() {
    let a = String::from("más");
    let b = String::from("sol");
    assert!(std::ptr::eq(mas_largo(&a, &b), a.as_str()));
    assert!(std::ptr::eq(mas_largo(&b, &a), b.as_str()));
}

#[test]
fn extracto_no_retiene_el_prestamo_del_contenedor() {
    let texto = String::from("Primera 🌳\nSegunda");
    let linea = {
        let extracto = Extracto { texto: &texto };
        extracto.primera_linea()
    };
    assert_eq!(linea, "Primera 🌳");
    assert_eq!(linea.as_ptr(), texto.as_ptr());
}

#[test]
fn primera_linea_vacia_y_crlf() {
    assert_eq!(Extracto { texto: "" }.primera_linea(), "");
    assert_eq!(Extracto { texto: "\nOtra" }.primera_linea(), "");
    assert_eq!(
        Extracto {
            texto: "uno\r\ndos"
        }
        .primera_linea(),
        "uno"
    );
}
