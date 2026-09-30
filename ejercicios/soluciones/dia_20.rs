pub fn suma_pares(numeros: &[i32]) -> i64 {
    numeros
        .iter()
        .filter(|n| **n % 2 == 0)
        .map(|n| i64::from(*n))
        .sum()
}
