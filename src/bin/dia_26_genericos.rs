fn maximo<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", maximo(10, 7));
    println!("{}", maximo(3.5, 4.2));
}
