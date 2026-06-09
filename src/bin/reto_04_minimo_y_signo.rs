fn minimo(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn signo(n: i32) -> &'static str {
    if n > 0 {
        "positivo"
    } else if n < 0 {
        "negativo"
    } else {
        "cero"
    }
}

fn main() {
    let a = -4;
    let b = 12;
    println!("Minimo entre {a} y {b}: {}", minimo(a, b));
    println!("Signo de {a}: {}", signo(a));
    println!("Signo de 0: {}", signo(0));
    println!("Signo de {b}: {}", signo(b));
}
