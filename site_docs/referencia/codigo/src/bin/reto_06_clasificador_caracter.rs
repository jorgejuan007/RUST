fn clasificar(c: char) -> &'static str {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => "vocal",
        '0'..='9' => "numero",
        'a'..='z' | 'A'..='Z' => "consonante",
        _ => "otro",
    }
}

fn main() {
    for c in ['a', 'R', '9', '#'] {
        println!("{c} -> {}", clasificar(c));
    }
}
