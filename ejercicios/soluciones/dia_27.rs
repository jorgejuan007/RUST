pub fn mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.chars().count() >= b.chars().count() {
        a
    } else {
        b
    }
}

pub struct Extracto<'a> {
    pub texto: &'a str,
}

impl<'a> Extracto<'a> {
    pub fn primera_linea(&self) -> &'a str {
        self.texto.lines().next().unwrap_or("")
    }
}
