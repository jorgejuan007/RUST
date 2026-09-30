from __future__ import annotations

import re
import shutil
import os
from pathlib import Path
from urllib.parse import quote, urlsplit, unquote


ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / "site_docs"
COURSE = DEST / "curso"
COMP = DEST / "complementos"
REF = DEST / "referencia"
REF_CODE = REF / "codigo"
REF_DATA = REF / "data"
REF_INFRA = REF / "infra"

REPO_URL = "https://github.com/jorgejuan007/RUST"
LINK_RE = re.compile(r"(?<!!)\[([^\]]+)\]\(([^)\n]+)\)")

def ensure_clean_dir(path: Path) -> None:
    if path.exists():
        shutil.rmtree(path)
    path.mkdir(parents=True, exist_ok=True)


def source_to_dest_map() -> dict[Path, Path]:
    mapping: dict[Path, Path] = {
        ROOT / "README.md": COURSE / "repositorio.md",
        ROOT / "manual_rust_30_dias.md": COURSE / "manual_30_dias.md",
        ROOT / "docs" / "README.md": COMP / "index.md",
        ROOT / "ejercicios" / "README.md": DEST / "practica" / "index.md",
        ROOT / "src" / "api" / "openapi.json": DEST / "assets" / "openapi.json",
    }

    for doc in sorted((ROOT / "docs").glob("*.md")):
        if doc.name == "README.md":
            continue
        mapping[doc] = COMP / doc.name

    return mapping
def rewrite_markdown(text: str, source: Path, mapping: dict[Path, Path]) -> str:
    source_dest = mapping[source]

    def repl(match: re.Match[str]) -> str:
        label, target = match.groups()
        url = urlsplit(target)
        if url.scheme or target.startswith(("#", "//")):
            return match.group(0)
        if target.startswith("/"):
            raise ValueError(f"Enlace no portable en {source.name}: {target}")
        resolved = (source.parent / unquote(url.path)).resolve()
        relative = resolved.relative_to(ROOT)
        if not resolved.exists():
            raise ValueError(f"Enlace inexistente en {source.name}: {target}")
        suffix = ("?" + url.query if url.query else "") + ("#" + url.fragment if url.fragment else "")
        if resolved in mapping:
            rendered = mapping[resolved]
            rel = Path(os.path.relpath(rendered, source_dest.parent)).as_posix()
            return f"[{label}]({quote(rel, safe='/')}{suffix})"
        return f"[{label}]({REPO_URL}/blob/main/{quote(relative.as_posix(), safe='/')}{suffix})"

    return LINK_RE.sub(repl, text)


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
- bonus practicos y una capa avanzada con `clap`, `serde`, `thiserror`, `tokio`, `reqwest`, `axum`, `csv`, `toml` y `rusqlite`

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
- [Backend local](complementos/23_backend_local_con_axum_y_reqwest.md)
- [Persistencia](complementos/24_persistencia_con_csv_y_toml.md)
- [SQLite](complementos/25_sqlite_con_rusqlite.md)
- [Backend modular](complementos/26_backend_axum_modular_y_storage.md)
- [Ejercicios con corrección automática](practica/index.md)
- [Proyecto final persistente](complementos/28_proyecto_tareas_persistente.md)
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
cargo run --bin bonus_backend_axum_reqwest
cargo run --bin bonus_persistencia_csv_toml
cargo run --bin bonus_sqlite_rusqlite
cargo run --bin bonus_backend_axum_modular
cargo run --bin bonus_backend_axum_cliente -- health
cargo run --bin proyecto_tareas_cli -- list
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

Esta seccion sirve como mapa rapido del codigo fuente, los datos de ejemplo y la infraestructura del repositorio.

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

El sitio web no duplica el codigo Rust. En su lugar, resume las rutas mas utiles del repositorio para que puedas localizar rapidamente cada ejemplo.

## Puntos de entrada utiles

- `src/lib.rs`
- `src/main.rs`
- `src/bin/bonus_gestor_tareas_json.rs`
- `src/bin/bonus_errores_io_anyhow.rs`
- `src/bin/bonus_smart_pointers.rs`
- `src/bin/bonus_async_tokio_avanzado.rs`
- `src/bin/bonus_backend_axum_reqwest.rs`
- `src/bin/bonus_persistencia_csv_toml.rs`
- `src/bin/bonus_sqlite_rusqlite.rs`
- `src/bin/bonus_backend_axum_modular/main.rs`
- `src/api/mod.rs`
- `src/api/storage.rs`
- `src/bin/bonus_backend_axum_cliente.rs`
- `src/bin/proyecto_tareas_cli.rs`
- `src/bin/reto_22_modularizado/main.rs`
- `src/bin/reto_30_cli_tareas.rs`
- `tests/integracion_curso.rs`
- `tests/api_tareas.rs`
- `tests/cli_tareas.rs`

## Nota

El codigo real vive en el repositorio local. Esta pagina funciona como indice de navegacion.
""",
        encoding="utf-8",
    )

    (REF / "data.md").write_text(
        """# Guia de Archivos de Datos

Los ejemplos del curso usan varios archivos de apoyo guardados en `data/` dentro del repositorio.

## Archivos destacados

- `data/bonus_tareas.json`
- `data/bonus_tareas.tsv`
- `data/bonus_numeros.txt`
- `data/bonus_tareas.csv`
- `data/bonus_config.toml`
- `data/dia_18_datos.txt`
- `data/dia_21_texto.txt`
- `data/reto_18_lineas.txt`
- `data/reto_30_tareas_cli.tsv`

## Nota

Estos archivos se referencian desde el curso y siguen viviendo en el repositorio local.
""",
        encoding="utf-8",
    )

    (REF / "infra.md").write_text(
        """# Guia de Automatizacion e Infraestructura

Esta seccion recopila los archivos de soporte para automatizacion local, CI y publicacion.

## Archivos clave

- `Cargo.toml`
- `Cargo.lock`
- `Makefile`
- `mkdocs.yml`
- `requirements-docs.txt`
- `scripts/sync_mkdocs.py`
- `.github/workflows/ci.yml`
- `.github/workflows/pages.yml`

## Nota

La infraestructura real se mantiene en el repositorio local. Esta pagina funciona como mapa rapido.
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
    for path in [COURSE, COMP, REF, REF_CODE, REF_DATA, REF_INFRA]:
        path.mkdir(parents=True, exist_ok=True)

    mapping = source_to_dest_map()

    for source, dest in mapping.items():
        dest.parent.mkdir(parents=True, exist_ok=True)
        if source.suffix != ".md":
            shutil.copyfile(source, dest)
            continue
        text = source.read_text(encoding="utf-8")
        dest.write_text(rewrite_markdown(text, source, mapping), encoding="utf-8")

    write_static_pages()


if __name__ == "__main__":
    main()
