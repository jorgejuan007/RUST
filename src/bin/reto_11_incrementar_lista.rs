fn incrementar(valores: &mut [i32]) {
    for valor in valores {
        *valor += 1;
    }
}

fn main() {
    let mut numeros = vec![1, 2, 3, 4];
    incrementar(&mut numeros);
    println!("Valores incrementados: {numeros:?}");
}
