fn main() {
    let dias = [
        "dia_01_hola_rust",
        "dia_02_variables_tipos",
        "dia_03_tuplas_arrays",
        "dia_04_funciones",
        "dia_05_control_flujo",
        "dia_06_match_if_let",
        "dia_07_calculadora_cli",
        "dia_08_string_vs_str",
        "dia_09_ownership",
        "dia_10_referencias_inmutables",
        "dia_11_referencias_mutables",
        "dia_12_slices",
        "dia_13_structs_metodos",
        "dia_14_tareas_memoria",
        "dia_15_enums",
        "dia_16_option",
        "dia_17_result",
        "dia_18_operador_q",
        "dia_19_colecciones",
        "dia_20_iteradores",
        "dia_21_analizador_texto",
        "dia_22_modulos",
        "dia_23_tests",
        "dia_24_pattern_matching",
        "dia_25_traits",
        "dia_26_genericos",
        "dia_27_lifetimes",
        "dia_28_libreria_reutilizable",
        "dia_29_concurrencia",
        "dia_30_proyecto_final",
    ];

    println!("Curso de Rust en 30 dias");
    println!();
    println!("Manual: manual_rust_30_dias.md");
    println!("Ejecuta un dia concreto con:");
    println!("cargo run --bin <nombre_del_binario>");
    println!();
    println!("Binarios disponibles:");

    for dia in dias {
        println!("- {dia}");
    }
}
