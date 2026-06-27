use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let contador = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for i in 1..=3 {
        let tx = tx.clone();
        let contador = Arc::clone(&contador);

        let handle = thread::spawn(move || {
            {
                let mut total = contador.lock().unwrap();
                *total += 1;
            }

            tx.send(format!("Mensaje desde hilo {i}")).unwrap();
        });

        handles.push(handle);
    }

    drop(tx);

    for mensaje in rx {
        println!("{mensaje}");
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Hilos completados: {}", *contador.lock().unwrap());
}
