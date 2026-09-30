use rust_practica::dia_26::maximo;

#[test]
fn vacio_y_un_elemento() {
    assert_eq!(maximo::<i32>(&[]), None);
    assert_eq!(maximo(&[-7]), Some(&-7));
}

#[test]
fn orden_y_numeros_negativos() {
    assert_eq!(maximo(&[-9, -2, -7]), Some(&-2));
    assert_eq!(maximo(&[8, 4, 1]), Some(&8));
}

#[test]
fn strings_se_prestan_y_no_se_consumen() {
    let valores = vec![String::from("z"), String::from("árbol"), String::from("a")];
    let mayor = maximo(&valores).unwrap();
    assert_eq!(mayor, "árbol");
    assert!(std::ptr::eq(mayor, &valores[1]));
    assert_eq!(valores.len(), 3);
}

#[test]
fn tipo_sin_clone_y_empate_conserva_el_primero() {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Valor(i64);
    let valores = [Valor(4), Valor(9), Valor(9)];
    assert!(std::ptr::eq(maximo(&valores).unwrap(), &valores[1]));
}
