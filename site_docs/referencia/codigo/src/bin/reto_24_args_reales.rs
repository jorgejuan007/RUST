use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Uso: cargo run --bin reto_24_args_reales -- add tarea");
        return;
    }

    println!("Argumentos recibidos: {args:?}");
}
