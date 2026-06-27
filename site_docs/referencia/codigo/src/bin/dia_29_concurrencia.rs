use std::thread;

fn main() {
    let h1 = thread::spawn(|| {
        for i in 1..=3 {
            println!("Hilo 1: {i}");
        }
    });

    let h2 = thread::spawn(|| {
        for i in 1..=3 {
            println!("Hilo 2: {i}");
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
