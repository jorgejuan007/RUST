use rust_30_dias::numeros;
use rust_30_dias::tareas::TaskManager;
use rust_30_dias::texto;

fn main() {
    let factorial = numeros::factorial(6);
    let serie = numeros::fibonacci(7);
    let primera = texto::primera_palabra("rust simplifica sistemas complejos");
    let estadisticas = texto::estadisticas("Rust es seguro.\nRust es rapido.");

    let mut tareas = TaskManager::new();
    tareas.agregar("Estudiar librerias");
    tareas.agregar("Escribir tests");
    tareas.completar(1);

    println!("Factorial de 6: {factorial}");
    println!("Fibonacci(7): {serie:?}");
    println!("Primera palabra: {primera}");
    println!("Estadisticas: {estadisticas:?}");
    println!("Pendientes: {}", tareas.pendientes());
}
