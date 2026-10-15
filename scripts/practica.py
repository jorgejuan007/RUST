"""Correccion de ejercicios sin instalar dependencias Python."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
EXERCISES = ROOT / "ejercicios"


def catalog() -> list[dict]:
    return json.loads((EXERCISES / "catalogo.json").read_text(encoding="utf-8"))


def pruebas_superadas(salida: str | None) -> int:
    """Cuenta las pruebas que pasaron leyendo la salida de `cargo test`."""
    if not salida:
        return 0
    return sum(int(total) for total in re.findall(r"(\d+) passed", salida))


def git(*argumentos: str) -> str | None:
    """Ejecuta git sobre el repositorio del curso; None si git o el historial no estan."""
    try:
        # Popen en lugar de check_output: las pruebas de este repositorio sustituyen
        # subprocess.run para simular cargo y no deben interceptar las llamadas a git.
        proceso = subprocess.Popen(
            ["git", "-C", str(ROOT), *argumentos],
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            text=True,
        )
        salida, _ = proceso.communicate()
    except OSError:
        return None

    if proceso.returncode != 0:
        return None

    return salida


def restaurar_pruebas(copy: Path) -> bool:
    """Recupera las pruebas confirmadas en git para que la nota no dependa de editarlas."""
    listado = git("ls-tree", "--name-only", "HEAD", "ejercicios/tests/")
    if not listado:
        return False

    ficheros = [nombre for nombre in listado.split() if nombre.endswith(".rs")]
    if not ficheros:
        return False

    for nombre in ficheros:
        contenido = git("show", f"HEAD:{nombre}")
        if contenido is None:
            return False
        (copy / "tests" / Path(nombre).name).write_text(contenido, encoding="utf-8")

    return True


def copia_de_ejercicios() -> tuple[tempfile.TemporaryDirectory, Path]:
    """Copia el paquete a una carpeta temporal: tus ficheros nunca se modifican."""
    temporary = tempfile.TemporaryDirectory(prefix="rust-practica-")
    copy = Path(temporary.name) / "ejercicios"
    shutil.copytree(EXERCISES, copy, ignore=shutil.ignore_patterns("target"))
    return temporary, copy


def cargo(arguments: list[str], copy: Path) -> subprocess.CompletedProcess | None:
    executable = shutil.which("cargo")
    if executable is None:
        print("Rust no esta disponible. Instala Rust desde https://rust-lang.org/tools/install/")
        return None

    environment = os.environ.copy()
    # Cada ejecucion compila en una carpeta nueva, dentro de la copia temporal. Compartir
    # el target entre copias distintas (alumno y soluciones) reutiliza artefactos obsoletos.
    environment["CARGO_TARGET_DIR"] = str(copy.parent / "target")
    return subprocess.run(
        [executable, *arguments, "--manifest-path", str(copy / "Cargo.toml")],
        env=environment,
        check=False,
        capture_output=True,
        text=True,
    )


def mostrar(completed: subprocess.CompletedProcess) -> int:
    """Reproduce la salida de cargo y decide si el ejercicio esta superado."""
    if completed.stdout:
        print(completed.stdout, end="")
    if completed.stderr:
        print(completed.stderr, end="", file=sys.stderr)

    if completed.returncode != 0:
        return completed.returncode

    if pruebas_superadas(completed.stdout) == 0:
        print("No se ejecuto ninguna prueba. Restaura las pruebas con:")
        print("git checkout -- ejercicios/tests")
        return 1

    return 0


def verificar_soluciones() -> int:
    # Se trabaja en una copia; nunca se sustituyen las respuestas del alumno.
    temporary, copy = copia_de_ejercicios()
    with temporary:
        restaurar_pruebas(copy)
        for solution in (copy / "soluciones").glob("*.rs"):
            shutil.copyfile(solution, copy / "src" / solution.name)
        completed = cargo(["test", "--locked"], copy)

    if completed is None:
        return 2
    return mostrar(completed)


def comprobar(dia: int, nombre: str) -> int:
    print(f"Comprobando dia {dia}. Edita ejercicios/src/{nombre}.rs", flush=True)
    temporary, copy = copia_de_ejercicios()
    with temporary:
        if not restaurar_pruebas(copy):
            print("Aviso: no se pudieron recuperar las pruebas originales desde git.")
            print("Se corrigen las pruebas que hay en ejercicios/tests/.")
        completed = cargo(["test", "--locked", "--test", nombre], copy)

    if completed is None:
        return 2

    resultado = mostrar(completed)
    print("Ejercicio superado." if resultado == 0 else "Revisa los errores y vuelve a intentarlo.")
    return resultado


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
        return verificar_soluciones()
    lesson = next(lesson for lesson in lessons if lesson["dia"] == args.dia)
    name = f"dia_{args.dia:02}"
    if args.command == "pista":
        print(lesson["enunciado"])
        print(f"Pista {args.nivel}: {lesson['pistas'][args.nivel - 1]}")
        return 0
    if args.command == "solucion":
        print((EXERCISES / "soluciones" / f"{name}.rs").read_text(encoding="utf-8"))
        return 0
    return comprobar(args.dia, name)


if __name__ == "__main__":
    raise SystemExit(main())
