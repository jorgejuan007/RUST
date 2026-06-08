fn celsius_a_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn area_rectangulo(base: f64, altura: f64) -> f64 {
    base * altura
}

fn main() {
    let temperatura_c: f64 = 25.0;
    let base = 5.0;
    let altura = 3.0;
    let activo = true;
    let inicial = 'R';

    let temperatura_f = celsius_a_fahrenheit(temperatura_c);
    let area = area_rectangulo(base, altura);

    println!("{temperatura_c}C = {temperatura_f}F");
    println!("Area del rectangulo: {area}");
    println!("Activo: {activo}");
    println!("Inicial: {inicial}");
}
