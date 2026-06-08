fn clasificar_nota(nota: u32) -> &'static str {
    match nota {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Nota invalida",
    }
}

fn main() {
    let nota = 84;
    println!("Clasificacion: {}", clasificar_nota(nota));
}
