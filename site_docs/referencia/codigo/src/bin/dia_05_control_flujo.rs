fn factorial(n: u32) -> u32 {
    let mut resultado = 1;
    for i in 1..=n {
        resultado *= i;
    }
    resultado
}

fn imprimir_tabla(n: u32) {
    for i in 1..=10 {
        println!("{n} x {i} = {}", n * i);
    }
}

fn main() {
    let numero = 5;
    println!("Factorial de {numero}: {}", factorial(numero));
    imprimir_tabla(numero);
}
