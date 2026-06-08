fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn restar(a: i32, b: i32) -> i32 {
    a - b
}

fn es_par(n: i32) -> bool {
    n % 2 == 0
}

fn maximo(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("Suma: {}", sumar(10, 5));
    println!("Resta: {}", restar(10, 5));
    println!("Es par 8: {}", es_par(8));
    println!("Maximo: {}", maximo(10, 5));
}
