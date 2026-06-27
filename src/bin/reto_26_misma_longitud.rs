fn misma_longitud<T, U>(a: &[T], b: &[U]) -> bool {
    a.len() == b.len()
}

fn main() {
    let numeros = [1, 2, 3];
    let palabras = ["uno", "dos", "tres"];
    let corta = ['a', 'b'];

    println!(
        "Numeros y palabras misma longitud: {}",
        misma_longitud(&numeros, &palabras)
    );
    println!(
        "Numeros y corta misma longitud: {}",
        misma_longitud(&numeros, &corta)
    );
}
