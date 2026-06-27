fn suma(valores: &[i32]) -> i32 {
    valores.iter().fold(0, |acc, n| acc + n)
}

fn maximo(valores: &[i32]) -> Option<i32> {
    valores.iter().copied().fold(None, |acc, n| match acc {
        Some(actual) if actual > n => Some(actual),
        _ => Some(n),
    })
}

fn main() {
    let numeros = [4, 9, 2, 11, 6];
    println!("Suma: {}", suma(&numeros));
    println!("Maximo: {:?}", maximo(&numeros));
}
