use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
enum Expr {
    Numero(i32),
    Suma(Box<Expr>, Box<Expr>),
}

fn evaluar(expr: &Expr) -> i32 {
    match expr {
        Expr::Numero(valor) => *valor,
        Expr::Suma(izquierda, derecha) => evaluar(izquierda) + evaluar(derecha),
    }
}

fn demo_box() {
    let expresion = Expr::Suma(
        Box::new(Expr::Numero(20)),
        Box::new(Expr::Suma(
            Box::new(Expr::Numero(7)),
            Box::new(Expr::Numero(5)),
        )),
    );

    println!("Box para tipos recursivos: {}", evaluar(&expresion));
}

fn demo_rc_refcell() {
    let plan = Rc::new(RefCell::new(vec!["ownership".to_string()]));
    let editor_a = Rc::clone(&plan);
    let editor_b = Rc::clone(&plan);

    editor_a.borrow_mut().push("traits".to_string());
    editor_b.borrow_mut().push("async".to_string());

    println!("Rc strong_count: {}", Rc::strong_count(&plan));
    println!("RefCell compartido en un hilo: {:?}", plan.borrow());
}

fn demo_cell() {
    struct Marcador {
        lecturas: Cell<u32>,
    }

    let marcador = Marcador {
        lecturas: Cell::new(0),
    };

    for _ in 0..3 {
        let actual = marcador.lecturas.get();
        marcador.lecturas.set(actual + 1);
    }

    println!(
        "Cell para valores Copy: {} lecturas",
        marcador.lecturas.get()
    );
}

fn demo_arc_mutex() {
    let contador = Arc::new(Mutex::new(0_u32));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let contador = Arc::clone(&contador);
        handles.push(thread::spawn(move || {
            let mut guard = contador.lock().expect("mutex envenenado");
            *guard += 1;
        }));
    }

    for handle in handles {
        handle.join().expect("hilo fallido");
    }

    println!(
        "Arc<Mutex<_>> entre hilos: {}",
        *contador.lock().expect("mutex envenenado")
    );
}

fn main() {
    println!("Smart pointers y mutabilidad interior");
    println!();
    demo_box();
    demo_rc_refcell();
    demo_cell();
    demo_arc_mutex();
}
