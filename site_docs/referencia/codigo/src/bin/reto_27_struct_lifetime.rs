struct Referencia<'a> {
    texto: &'a str,
}

impl<'a> Referencia<'a> {
    fn ver(&self) -> &'a str {
        self.texto
    }
}

fn main() {
    let frase = String::from("ownership y lifetimes");
    let referencia = Referencia { texto: &frase };
    println!("Texto prestado: {}", referencia.ver());
}
