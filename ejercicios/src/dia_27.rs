//! Relaciona los préstamos de entrada y salida sin crear Strings nuevos.

#[allow(unused_variables)]
pub fn mas_largo<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!("Cuenta caracteres Unicode y devuelve a en caso de empate")
}

pub struct Extracto<'a> {
    pub texto: &'a str,
}

impl<'a> Extracto<'a> {
    pub fn primera_linea(&self) -> &'a str {
        todo!("Devuelve la primera línea del texto prestado, o un slice vacío")
    }
}
