.PHONY: help sync-docs docs-serve docs-build check-links fmt lint check test test-doc test-python test-practice doc-api ci practica practica-lista

DIA ?= 9

help:
	@echo "make ci: formato, Clippy, pruebas, ejercicios y documentación"
	@echo "make docs-serve / docs-build / sync-docs / check-links"
	@echo "make fmt / lint / check / test / test-doc / test-python / test-practice / doc-api"
	@echo "make practica DIA=9 / practica-lista"

sync-docs:
	python3 scripts/sync_mkdocs.py

check-links:
	python3 scripts/check_docs.py

docs-serve: sync-docs
	mkdocs serve

docs-build: check-links sync-docs
	mkdocs build --strict

fmt:
	cargo fmt --all -- --check
	cargo fmt --manifest-path ejercicios/Cargo.toml -- --check

lint:
	cargo clippy --locked --all-targets -- -D warnings

check:
	cargo check --locked --all-targets
	cargo check --locked --manifest-path ejercicios/Cargo.toml

test:
	cargo test --locked

test-doc:
	cargo test --locked --doc

test-python:
	python3 -m unittest discover -s tests_python -v

test-practice:
	python3 scripts/practica.py verificar-soluciones

practica:
	python3 scripts/practica.py comprobar $(DIA)

practica-lista:
	python3 scripts/practica.py listar

doc-api:
	cargo doc --locked --no-deps

ci:
	$(MAKE) fmt
	$(MAKE) lint
	$(MAKE) check
	$(MAKE) test
	$(MAKE) test-python
	$(MAKE) test-practice
	$(MAKE) docs-build
