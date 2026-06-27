fn fahrenheit_a_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn area_triangulo(base: f64, altura: f64) -> f64 {
    base * altura / 2.0
}

fn main() {
    let fahrenheit = 86.0;
    let base = 10.0;
    let altura = 5.0;

    println!("{fahrenheit}F = {}C", fahrenheit_a_celsius(fahrenheit));
    println!(
        "Area del triangulo con base {base} y altura {altura}: {}",
        area_triangulo(base, altura)
    );
}
