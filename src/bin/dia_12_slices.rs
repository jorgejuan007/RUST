fn primera_palabra(texto: &str) -> &str {
    for (i, byte) in texto.as_bytes().iter().enumerate() {
        if *byte == b' ' {
            return &texto[0..i];
        }
    }
    texto
}

fn main() {
    let frase = "rust es potente";
    let primera = primera_palabra(frase);
    println!("{primera}");
}
