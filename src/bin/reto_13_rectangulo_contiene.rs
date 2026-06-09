struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.alto > otro.alto
    }
}

fn main() {
    let grande = Rectangulo { ancho: 20, alto: 15 };
    let pequeno = Rectangulo { ancho: 8, alto: 7 };
    let ancho = Rectangulo { ancho: 22, alto: 6 };

    println!("Grande contiene pequeno: {}", grande.puede_contener(&pequeno));
    println!("Grande contiene ancho: {}", grande.puede_contener(&ancho));
}
