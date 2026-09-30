use rust_practica::dia_25::{resumir, Libro, Resumen, Tarea};

#[test]
fn libro_y_unicode() {
    let libro = Libro {
        titulo: "Árboles 🌳".into(),
        autor: "María".into(),
    };
    assert_eq!(libro.resumen(), "Libro: Árboles 🌳 (María)");
    assert_eq!(resumir(&libro), libro.resumen());
    assert_eq!(libro.titulo, "Árboles 🌳");
}

#[test]
fn tarea_en_ambos_estados() {
    let mut tarea = Tarea {
        titulo: "Practicar traits".into(),
        hecha: false,
    };
    assert_eq!(resumir(&tarea), "Pendiente: Practicar traits");
    tarea.hecha = true;
    assert_eq!(resumir(&tarea), "Hecha: Practicar traits");
}

#[test]
fn otro_tipo_que_implementa_el_trait() {
    struct Aviso;
    impl Resumen for Aviso {
        fn resumen(&self) -> String {
            "También funciona".into()
        }
    }
    assert_eq!(resumir(&Aviso), "También funciona");
}

#[test]
fn campos_vacios_conservan_el_formato() {
    let libro = Libro {
        titulo: String::new(),
        autor: String::new(),
    };
    assert_eq!(resumir(&libro), "Libro:  ()");
}
