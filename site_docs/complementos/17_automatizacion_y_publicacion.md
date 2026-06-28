# Automatizacion y Publicacion

Este documento resume como trabajar el curso con comandos repetibles y como publicar el sitio MkDocs.

## Automatizacion local

El repositorio incluye un Makefile (`Makefile`) con objetivos utiles.

### Objetivos principales

```bash
make help
make sync-docs
make docs-serve
make docs-build
make check
make test
make test-doc
make doc-api
make ci
```

## Que hace `make ci`

Ejecuta esta secuencia:

1. regenera `site_docs`
2. corre `cargo check --bins`
3. corre `cargo test`
4. construye MkDocs con `--strict`

## Dependencias de documentacion

Para mantener reproducible la capa web, MkDocs queda fijado en:

- requirements-docs.txt (`requirements-docs.txt`)

## GitHub Actions

Hay dos workflows:

- CI (`.github/workflows/ci.yml`)
- GitHub Pages (`.github/workflows/pages.yml`)

### CI

Valida:

- sincronizacion de `site_docs`
- `cargo check --bins`
- `cargo test`
- `mkdocs build --strict`

### Pages

Regenera `site_docs`, construye el sitio y publica el directorio `site` en GitHub Pages.

## Flujo recomendado

Antes de subir cambios:

```bash
make ci
```

Si estas trabajando contenido web:

```bash
make docs-serve
```

## Nota final

Esta capa no sustituye el estudio de Rust, pero si mejora mucho la mantenibilidad del curso y evita regresiones silenciosas en la documentacion.
