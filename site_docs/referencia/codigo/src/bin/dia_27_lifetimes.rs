fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = "corto";
    let b = "mucho mas largo";
    println!("{}", mas_largo(a, b));
}
