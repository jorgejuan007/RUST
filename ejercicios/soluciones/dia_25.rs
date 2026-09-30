pub trait Resumen {
    fn resumen(&self) -> String;
}

pub struct Libro {
    pub titulo: String,
    pub autor: String,
}

pub struct Tarea {
    pub titulo: String,
    pub hecha: bool,
}

impl Resumen for Libro {
    fn resumen(&self) -> String {
        format!("Libro: {} ({})", self.titulo, self.autor)
    }
}

impl Resumen for Tarea {
    fn resumen(&self) -> String {
        let estado = if self.hecha { "Hecha" } else { "Pendiente" };
        format!("{estado}: {}", self.titulo)
    }
}

pub fn resumir(item: &impl Resumen) -> String {
    item.resumen()
}
