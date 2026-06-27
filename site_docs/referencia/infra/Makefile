.PHONY: help sync-docs docs-serve docs-build check test test-doc doc-api ci

help:
	@echo "Objetivos disponibles:"
	@echo "  make sync-docs   - regenera site_docs desde el contenido del repo"
	@echo "  make docs-serve  - sirve MkDocs en local"
	@echo "  make docs-build  - construye MkDocs en modo estricto"
	@echo "  make check       - corre cargo check --bins"
	@echo "  make test        - corre cargo test"
	@echo "  make test-doc    - corre solo los doctests"
	@echo "  make doc-api     - genera la documentacion Rust local"
	@echo "  make ci          - ejecuta sync-docs, check, test y docs-build"

sync-docs:
	python3 scripts/sync_mkdocs.py

docs-serve: sync-docs
	mkdocs serve

docs-build: sync-docs
	mkdocs build --strict

check:
	cargo check --bins

test:
	cargo test

test-doc:
	cargo test --doc

doc-api:
	cargo doc --no-deps

ci: sync-docs check test docs-build
