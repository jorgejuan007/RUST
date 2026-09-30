//! Implementa un contrato común y úsalo sin conocer el tipo concreto.

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
        todo!("Devuelve Libro: <titulo> (<autor>)")
    }
}

impl Resumen for Tarea {
    fn resumen(&self) -> String {
        todo!("Devuelve Hecha: <titulo> o Pendiente: <titulo>")
    }
}

#[allow(unused_variables)]
pub fn resumir(item: &impl Resumen) -> String {
    todo!("Usa el método del trait sin inspeccionar el tipo concreto")
}
