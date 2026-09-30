"""Comprueba los enlaces fuente, incluso antes de generar MkDocs."""
from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
LINK_RE = re.compile(r"(?<!!)\[([^\]]+)\]\(([^)\n]+)\)")


def check(root: Path = ROOT) -> list[str]:
    files = [root / "README.md", root / "manual_rust_30_dias.md", root / "ejercicios" / "README.md", *sorted((root / "docs").glob("*.md"))]
    errors = []
    for source in files:
        if not source.is_file():
            errors.append(f"Falta {source.relative_to(root)}")
            continue
        for _, target in LINK_RE.findall(source.read_text(encoding="utf-8")):
            url = urlsplit(target)
            if target.startswith("/") or url.scheme.lower() == "file" or re.match(r"^[A-Za-z]:[\\/]", target):
                errors.append(f"{source.relative_to(root)}: enlace no portable {target}")
                continue
            if url.scheme or target.startswith("#"):
                continue
            resolved = (source.parent / unquote(url.path)).resolve()
            if not resolved.is_relative_to(root.resolve()):
                errors.append(f"{source.relative_to(root)}: enlace fuera del repositorio {target}")
            elif not resolved.exists():
                errors.append(f"{source.relative_to(root)}: enlace inexistente {target}")
    return errors


if __name__ == "__main__":
    errors = check()
    for error in errors:
        print(error)
    print(f"Enlaces fuente: {'correctos' if not errors else str(len(errors)) + ' errores'}")
    raise SystemExit(bool(errors))
