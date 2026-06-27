from __future__ import annotations

import re
import shutil
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / "site_docs"
COURSE = DEST / "curso"
COMP = DEST / "complementos"
REF = DEST / "referencia"
REF_CODE = REF / "codigo"
REF_DATA = REF / "data"
REF_INFRA = REF / "infra"
REF_INFRA_WORKFLOWS = REF_INFRA / "workflows"
REF_INFRA_SCRIPTS = REF_INFRA / "scripts"

ABS_LINK_RE = re.compile(r"\[([^\]]+)\]\((/Users/legalintermedia/Documents/GitHub/RUST[^)]+)\)")


def ensure_clean_dir(path: Path) -> None:
    if path.exists():
        shutil.rmtree(path)
    path.mkdir(parents=True, exist_ok=True)


def source_to_dest_map() -> dict[Path, Path]:
    mapping: dict[Path, Path] = {
        ROOT / "README.md": COURSE / "repositorio.md",
        ROOT / "manual_rust_30_dias.md": COURSE / "manual_30_dias.md",
        ROOT / "docs" / "README.md": COMP / "index.md",
    }

    for doc in sorted((ROOT / "docs").glob("*.md")):
        if doc.name == "README.md":
            continue
        mapping[doc] = COMP / doc.name

    return mapping


def copy_reference_files() -> None:
    for src_file in (ROOT / "src").rglob("*.rs"):
        dest_file = REF_CODE / "src" / src_file.relative_to(ROOT / "src")
        dest_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src_file, dest_file)

    for test_file in (ROOT / "tests").rglob("*.rs"):
        dest_file = REF_CODE / "tests" / test_file.relative_to(ROOT / "tests")
        dest_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(test_file, dest_file)

    for data_file in (ROOT / "data").rglob("*"):
        if data_file.is_dir():
            continue
        dest_file = REF_DATA / data_file.relative_to(ROOT / "data")
        dest_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(data_file, dest_file)

    flat_infra_files = [
        ROOT / "Cargo.toml",
        ROOT / "Cargo.lock",
        ROOT / "Makefile",
        ROOT / "mkdocs.yml",
        ROOT / "requirements-docs.txt",
    ]

    for infra_file in flat_infra_files:
        dest_file = REF_INFRA / infra_file.relative_to(ROOT)
        dest_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(infra_file, dest_file)

    workflow_files = {
        ROOT / ".github" / "workflows" / "ci.yml": REF_INFRA_WORKFLOWS / "ci.yml",
        ROOT / ".github" / "workflows" / "pages.yml": REF_INFRA_WORKFLOWS / "pages.yml",
    }

    for infra_file, dest_file in workflow_files.items():
        dest_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(infra_file, dest_file)

    for script_file in (ROOT / "scripts").rglob("*.py"):
        dest_file = REF_INFRA_SCRIPTS / script_file.relative_to(ROOT / "scripts")
        dest_file.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(script_file, dest_file)


def target_for_abs_path(abs_path: Path, mapping: dict[Path, Path]) -> Path:
    if abs_path in mapping:
        return mapping[abs_path]

    if abs_path.is_relative_to(ROOT / "src"):
        return REF_CODE / "src" / abs_path.relative_to(ROOT / "src")

    if abs_path.is_relative_to(ROOT / "tests"):
        return REF_CODE / "tests" / abs_path.relative_to(ROOT / "tests")

    if abs_path.is_relative_to(ROOT / "data"):
        return REF_DATA / abs_path.relative_to(ROOT / "data")

    flat_infra_files = {
        ROOT / "Cargo.toml",
        ROOT / "Cargo.lock",
        ROOT / "Makefile",
        ROOT / "mkdocs.yml",
        ROOT / "requirements-docs.txt",
    }

    if abs_path in flat_infra_files:
        return REF_INFRA / abs_path.relative_to(ROOT)

    workflow_files = {
        ROOT / ".github" / "workflows" / "ci.yml": REF_INFRA_WORKFLOWS / "ci.yml",
        ROOT / ".github" / "workflows" / "pages.yml": REF_INFRA_WORKFLOWS / "pages.yml",
    }

    if abs_path in workflow_files:
        return workflow_files[abs_path]

    if abs_path.is_relative_to(ROOT / "scripts"):
        return REF_INFRA_SCRIPTS / abs_path.relative_to(ROOT / "scripts")

    raise ValueError(f"No se pudo mapear el enlace absoluto: {abs_path}")


def rewrite_markdown(text: str, source: Path, mapping: dict[Path, Path]) -> str:
    source_dest = mapping[source]

    def repl(match: re.Match[str]) -> str:
        label, target = match.groups()
        abs_target = Path(target)
        rendered = target_for_abs_path(abs_target, mapping)
        rel = Path(shutil.os.path.relpath(rendered, source_dest.parent)).as_posix()
        return f"[{label}]({rel})"

    return ABS_LINK_RE.sub(repl, text)


def write_static_pages() -> None:
    (DEST / "assets" / "stylesheets").mkdir(parents=True, exist_ok=True)
    (DEST / "index.md").write_text(
        """# Curso de Rust en 30 Dias

Bienvenido al sitio del curso. Esta version web organiza el mismo contenido del repositorio en una navegacion mas clara para estudiar.

## Que contiene

- un manual principal de 30 dias
- documentacion complementaria por temas
- retos extra
- soluciones orientativas
- soluciones completas ejecutables
- bonus practicos y una capa avanzada con `clap`, `serde`, `thiserror` y `tokio`

## Ruta recomendada

1. Empieza por la vista general del curso.
2. Sigue con el manual de 30 dias.
3. Usa los complementos para reforzar conceptos.
4. Resuelve los retos extra.
5. Consulta las soluciones solo despues de intentarlo.

## Atajos utiles

- [Vista general del curso](curso/index.md)
- [Manual 30 dias](curso/manual_30_dias.md)
- [Indice de complementos](complementos/index.md)
- [Soluciones ejecutables](complementos/16_soluciones_completas_ejecutables.md)
- [Binarios y comandos](referencia/binarios.md)
- [Automatizacion y publicacion](complementos/17_automatizacion_y_publicacion.md)
- [Ruta intermedia](complementos/18_errores_io_y_anyhow.md)
""",
        encoding="utf-8",
    )

    (COURSE / "index.md").write_text(
        """# Vista General del Curso

Este sitio deriva del repositorio del curso y organiza el contenido para estudio secuencial.

## Componentes principales

- [Guia del repositorio](repositorio.md)
- [Manual 30 dias](manual_30_dias.md)
- [Complementos](../complementos/index.md)
- [Binarios y comandos](../referencia/binarios.md)

## Como estudiar

La progresion sugerida es:

1. leer el dia correspondiente
2. ejecutar su binario
3. resolver el reto del dia
4. comparar con la solucion completa si hace falta
""",
        encoding="utf-8",
    )

    (REF / "binarios.md").write_text(
        """# Binarios y Comandos

Esta pagina resume como ejecutar el material practico desde la raiz del repositorio.

## Curso diario

```bash
cargo run --bin dia_01_hola_rust
cargo run --bin dia_07_calculadora_cli
cargo run --bin dia_18_operador_q
cargo run --bin dia_30_proyecto_final
```

## Bonus

```bash
cargo run --bin bonus_gestor_tareas_std -- list
cargo run --bin bonus_gestor_tareas_json -- list
cargo run --bin bonus_uso_libreria
cargo run --bin bonus_option_result_combinadores
cargo run --bin bonus_canales_mutex
cargo run --bin bonus_async_tokio
cargo run --bin bonus_errores_io_anyhow
cargo run --bin bonus_smart_pointers
cargo run --bin bonus_async_tokio_avanzado
```

## Retos completos

```bash
cargo run --bin reto_01_operaciones_extra
cargo run --bin reto_22_modularizado
cargo run --bin reto_30_cli_tareas -- list
```

## Verificaciones

```bash
cargo check --bins
cargo test
cargo test --doc
make ci
mkdocs build --strict
```
""",
        encoding="utf-8",
    )

    (REF / "index.md").write_text(
        """# Referencia de Codigo y Datos

Para mantener este sitio navegable, el codigo fuente y los archivos de datos usados por el curso se copian dentro del arbol de MkDocs.

## Rutas disponibles

- codigo Rust: [Guia de codigo fuente](codigo.md)
- datos de ejemplo: [Guia de archivos de datos](data.md)

## Uso recomendado

Consulta esta seccion cuando un documento del curso te mande a revisar un archivo `.rs` o uno de los datos de ejemplo.
""",
        encoding="utf-8",
    )

    (REF / "codigo.md").write_text(
        """# Guia de Codigo Fuente

El sitio copia el codigo del repositorio bajo `referencia/codigo/src/` para que los enlaces desde la documentacion funcionen en web.

## Puntos de entrada utiles

- [src/lib.rs](codigo/src/lib.rs)
- [src/main.rs](codigo/src/main.rs)
- [src/bin/bonus_gestor_tareas_json.rs](codigo/src/bin/bonus_gestor_tareas_json.rs)
- [src/bin/bonus_errores_io_anyhow.rs](codigo/src/bin/bonus_errores_io_anyhow.rs)
- [src/bin/bonus_smart_pointers.rs](codigo/src/bin/bonus_smart_pointers.rs)
- [src/bin/bonus_async_tokio_avanzado.rs](codigo/src/bin/bonus_async_tokio_avanzado.rs)
- [src/bin/reto_22_modularizado/main.rs](codigo/src/bin/reto_22_modularizado/main.rs)
- [src/bin/reto_30_cli_tareas.rs](codigo/src/bin/reto_30_cli_tareas.rs)
- [tests/integracion_curso.rs](codigo/tests/integracion_curso.rs)

## Nota

No todos los archivos se listan aqui, pero todo `src/` y `tests/` se copian al sitio.
""",
        encoding="utf-8",
    )

    (REF / "data.md").write_text(
        """# Guia de Archivos de Datos

Los ejemplos del curso usan varios archivos de apoyo copiados bajo `referencia/data/`.

## Archivos destacados

- [bonus_tareas.json](data/bonus_tareas.json)
- [bonus_tareas.tsv](data/bonus_tareas.tsv)
- [bonus_numeros.txt](data/bonus_numeros.txt)
- [dia_18_datos.txt](data/dia_18_datos.txt)
- [dia_21_texto.txt](data/dia_21_texto.txt)
- [reto_18_lineas.txt](data/reto_18_lineas.txt)
- [reto_30_tareas_cli.tsv](data/reto_30_tareas_cli.tsv)
""",
        encoding="utf-8",
    )

    (REF / "infra.md").write_text(
        """# Guia de Automatizacion e Infraestructura

Esta seccion recopila los archivos de soporte para automatizacion local, CI y publicacion.

## Archivos clave

- [Cargo.toml](infra/Cargo.toml)
- [Cargo.lock](infra/Cargo.lock)
- [Makefile](infra/Makefile)
- [mkdocs.yml](infra/mkdocs.yml)
- [requirements-docs.txt](infra/requirements-docs.txt)
- [sync_mkdocs.py](infra/scripts/sync_mkdocs.py)
- [CI workflow](infra/workflows/ci.yml)
- [Pages workflow](infra/workflows/pages.yml)
""",
        encoding="utf-8",
    )

    (DEST / "assets" / "stylesheets" / "extra.css").write_text(
        """:root {
  --accent: #a03c2f;
  --ink: #1f2a34;
  --paper: #fbf8f1;
}

body {
  color: var(--ink);
  background: var(--paper);
}

.wy-side-nav-search,
.wy-nav-top {
  background: linear-gradient(135deg, #a03c2f, #d78432);
}

.wy-menu-vertical header,
.wy-menu-vertical p.caption {
  color: #7a2d24;
}

a {
  color: #8c3429;
}

a:hover {
  color: #c5661f;
}

code {
  color: #8c3429;
}
""",
        encoding="utf-8",
    )


def main() -> None:
    ensure_clean_dir(DEST)
    for path in [
        COURSE,
        COMP,
        REF,
        REF_CODE,
        REF_DATA,
        REF_INFRA,
        REF_INFRA_WORKFLOWS,
        REF_INFRA_SCRIPTS,
    ]:
        path.mkdir(parents=True, exist_ok=True)

    mapping = source_to_dest_map()

    for source, dest in mapping.items():
        dest.parent.mkdir(parents=True, exist_ok=True)
        text = source.read_text(encoding="utf-8")
        dest.write_text(rewrite_markdown(text, source, mapping), encoding="utf-8")

    copy_reference_files()
    write_static_pages()


if __name__ == "__main__":
    main()
