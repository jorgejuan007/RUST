# Automatización y publicación fiable

## Preparación local

Instala Rust con rustup y Python 3.12 o posterior. La versión de Rust y sus componentes
se fijan en [rust-toolchain.toml](https://github.com/jorgejuan007/RUST/blob/main/rust-toolchain.toml).

```bash
python3 -m venv .venv
# macOS y Linux
source .venv/bin/activate
# Windows PowerShell: .venv\Scripts\Activate.ps1
python -m pip install -r requirements-docs.txt
make ci
```

El [Makefile](https://github.com/jorgejuan007/RUST/blob/main/Makefile) comprueba formato, Clippy, compilación, pruebas Rust,
pruebas Python, soluciones de los ejercicios y la construcción estricta de MkDocs.
En Windows también puedes ejecutar cada comando directamente, sin instalar make.

## Enlaces portables

README y las guías utilizan enlaces relativos al archivo fuente. El
[generador](https://github.com/jorgejuan007/RUST/blob/main/scripts/sync_mkdocs.py) los convierte a las páginas web correspondientes;
los enlaces a código y datos apuntan a archivos reales en GitHub.

```bash
python3 scripts/check_docs.py
python3 scripts/sync_mkdocs.py
mkdocs build --strict
```

El comprobador rechaza enlaces a rutas del ordenador, archivos inexistentes y rutas
fuera del repositorio. MkDocs eleva los problemas de enlaces internos a advertencias
que hacen fallar la construcción estricta. Una prueba adicional regenera el curso
desde otra carpeta para comprobar que no depende de la ruta original.

`site_docs/` se genera automáticamente. Edita README, el manual, `docs/`, las instrucciones
de ejercicios o el generador; evita editar directamente el contenido generado.

## CI

El [workflow CI](https://github.com/jorgejuan007/RUST/blob/main/.github/workflows/ci.yml) ejecuta las comprobaciones Rust en Linux,
Windows y macOS. Un job independiente valida la documentación y los ejercicios y
sube el sitio construido como artefacto `docs-site`.

## Activar GitHub Pages: paso administrativo inicial

El propietario o una persona con permisos de administración debe abrir el repositorio
en GitHub y seleccionar **Settings → Pages → Build and deployment → Source → GitHub Actions**.
El permiso de escritura de código no basta para habilitar Pages. El workflow no solicita
tokens adicionales ni cambia los permisos del repositorio.

Después de activarlo, el [workflow Pages](https://github.com/jorgejuan007/RUST/blob/main/.github/workflows/pages.yml) publica el
artefacto construido por un CI exitoso de un push a `main` o `master`. Comprueba el
repositorio, el evento, la rama, el workflow y su resultado antes de descargar el sitio.
No publica contenido de pull requests.

Si el despliegue falló porque Pages estaba desactivado, vuelve a ejecutar CI en la rama
principal. También puedes ejecutar Pages manualmente indicando el identificador de
una ejecución CI exitosa de esa rama. El sitio publicado corresponde a ese artefacto
validado, sin reconstruir una revisión diferente.

La publicación solo se considera comprobada cuando termina el job `deploy` y la URL
responde. Construir MkDocs por sí solo no demuestra que la web se haya publicado.
