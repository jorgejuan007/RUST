"""Correccion de ejercicios sin instalar dependencias Python."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
EXERCISES = ROOT / "ejercicios"


def catalog() -> list[dict]:
    return json.loads((EXERCISES / "catalogo.json").read_text(encoding="utf-8"))


def cargo(arguments: list[str], directory: Path = EXERCISES) -> int:
    executable = shutil.which("cargo")
    if executable is None:
        print("Rust no esta disponible. Instala Rust desde https://rust-lang.org/tools/install/")
        return 2
    return subprocess.run([executable, *arguments, "--manifest-path", str(directory / "Cargo.toml")], check=False).returncode


def verify_solutions() -> int:
    # Se trabaja en una copia; nunca se sustituyen las respuestas del alumno.
    with tempfile.TemporaryDirectory(prefix="rust-practica-") as temporary:
        copy = Path(temporary) / "ejercicios"
        shutil.copytree(EXERCISES, copy, ignore=shutil.ignore_patterns("target"))
        for solution in (copy / "soluciones").glob("*.rs"):
            shutil.copyfile(solution, copy / "src" / solution.name)
        environment = os.environ.copy()
        environment["CARGO_TARGET_DIR"] = str(ROOT / "target" / "practica-soluciones")
        executable = shutil.which("cargo")
        if executable is None:
            print("Rust no esta disponible.")
            return 2
        return subprocess.run([executable, "test", "--locked", "--manifest-path", str(copy / "Cargo.toml")], env=environment, check=False).returncode


def main() -> int:
    lessons = catalog()
    parser = argparse.ArgumentParser(description="Practica Rust con correccion automatica")
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("listar")
    commands.add_parser("verificar-soluciones")
    for command in ["comprobar", "pista", "solucion"]:
        child = commands.add_parser(command)
        child.add_argument("dia", type=int, choices=[lesson["dia"] for lesson in lessons])
        if command == "pista":
            child.add_argument("--nivel", type=int, choices=[1, 2, 3], default=1)
    args = parser.parse_args()
    if args.command == "listar":
        for lesson in lessons:
            print(f"Dia {lesson['dia']:02}: {lesson['titulo']}")
        return 0
    if args.command == "verificar-soluciones":
        return verify_solutions()
    lesson = next(lesson for lesson in lessons if lesson["dia"] == args.dia)
    name = f"dia_{args.dia:02}"
    if args.command == "pista":
        print(lesson["enunciado"])
        print(f"Pista {args.nivel}: {lesson['pistas'][args.nivel - 1]}")
        return 0
    if args.command == "solucion":
        print((EXERCISES / "soluciones" / f"{name}.rs").read_text(encoding="utf-8"))
        return 0
    print(f"Comprobando dia {args.dia}. Edita ejercicios/src/{name}.rs", flush=True)
    result = cargo(["test", "--locked", "--test", name])
    print("Ejercicio superado." if result == 0 else "Revisa los errores y vuelve a intentarlo.")
    return result


if __name__ == "__main__":
    raise SystemExit(main())
