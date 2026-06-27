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
    let bonus = [
        "bonus_gestor_tareas_std",
        "bonus_gestor_tareas_json",
        "bonus_uso_libreria",
        "bonus_option_result_combinadores",
        "bonus_canales_mutex",
        "bonus_async_tokio",
        "bonus_errores_io_anyhow",
        "bonus_smart_pointers",
        "bonus_async_tokio_avanzado",
    ];
    let retos = [
        "reto_01_operaciones_extra",
        "reto_02_conversiones_y_triangulo",
        "reto_03_mediana_array",
        "reto_04_minimo_y_signo",
        "reto_05_fibonacci",
        "reto_06_clasificador_caracter",
        "reto_07_calculadora_segura",
        "reto_08_nombre_completo",
        "reto_09_move_vs_prestamo",
        "reto_10_firmas_str",
        "reto_11_incrementar_lista",
        "reto_12_ultima_palabra",
        "reto_13_rectangulo_contiene",
        "reto_14_tareas_pendientes",
        "reto_15_estado_bloqueada",
        "reto_16_buscar_tarea_mut",
        "reto_17_parseo_errores",
        "reto_18_lineas_no_vacias",
        "reto_19_conteo_normalizado",
        "reto_20_fold_suma_maximo",
        "reto_21_palabra_mas_frecuente",
        "reto_22_modularizado",
        "reto_23_test_palabras_vacias",
        "reto_24_args_reales",
        "reto_25_trait_identificable",
        "reto_26_misma_longitud",
        "reto_27_struct_lifetime",
        "reto_28_busqueda_task_manager",
        "reto_29_canal_mensajes",
        "reto_30_cli_tareas",
    ];

    println!("Curso de Rust en 30 dias");
    println!();
    println!("Manual: manual_rust_30_dias.md");
    println!("Documentacion ampliada: docs/README.md");
    println!("Soluciones completas de retos: docs/16_soluciones_completas_ejecutables.md");
    println!("Ejecuta un dia concreto con:");
    println!("cargo run --bin <nombre_del_binario>");
    println!("Proyecto bonus:");
    println!("cargo run --bin bonus_gestor_tareas_std -- list");
    println!("cargo run --bin bonus_gestor_tareas_json -- list");
    println!("cargo run --bin bonus_async_tokio");
    println!("cargo run --bin bonus_errores_io_anyhow");
    println!("cargo run --bin bonus_smart_pointers");
    println!("cargo run --bin bonus_async_tokio_avanzado");
    println!("Retos completos:");
    println!("cargo run --bin reto_01_operaciones_extra");
    println!("cargo run --bin reto_22_modularizado");
    println!("cargo run --bin reto_30_cli_tareas -- list");
    println!("Automatizacion local:");
    println!("make ci");
    println!("cargo test --doc");
    println!("cargo doc --no-deps");
    println!("MkDocs:");
    println!("python3 scripts/sync_mkdocs.py");
    println!("mkdocs serve");
    println!();
    println!("Binarios disponibles:");

    for dia in dias {
        println!("- {dia}");
    }

    for extra in bonus {
        println!("- {extra}");
    }

    println!();
    println!("Retos completos disponibles:");

    for reto in retos {
        println!("- {reto}");
    }
}
