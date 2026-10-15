from pathlib import Path
import contextlib
import io
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
from test_documentacion import load, ROOT


class PracticeTests(unittest.TestCase):
    def command(self, *args):
        return subprocess.run([sys.executable, str(ROOT / "scripts/practica.py"), *args], capture_output=True, text=True)

    def test_catalog_and_three_hints(self):
        catalog = load("practica").catalog()
        self.assertEqual([lesson["dia"] for lesson in catalog], [9, 10, 11, 12, 16, 17, 18, 19, 20, 25, 26, 27])
        for lesson in catalog:
            self.assertEqual(len(lesson["pistas"]), 3)
        result = self.command("pista", "12", "--nivel", "2")
        self.assertEqual(result.returncode, 0)
        self.assertIn("Pista 2", result.stdout)

    def test_invalid_day_is_rejected(self):
        self.assertNotEqual(self.command("comprobar", "99").returncode, 0)

    def test_solution_lookup_never_overwrites_student_file(self):
        path = ROOT / "ejercicios/src/dia_09.rs"
        before = path.read_bytes()
        self.assertEqual(self.command("solucion", "9").returncode, 0)
        self.assertEqual(path.read_bytes(), before)

    def test_verification_uses_a_separate_copy(self):
        module = load("practica")
        before = {p: p.read_bytes() for p in (module.EXERCISES / "src").glob("*.rs")}

        def inspect(arguments, **kwargs):
            copied = Path(arguments[-1]).parent
            self.assertNotEqual(copied, module.EXERCISES)
            self.assertNotIn("todo!", (copied / "src/dia_09.rs").read_text())
            return subprocess.CompletedProcess(
                arguments,
                0,
                stdout="test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out",
                stderr="",
            )

        with patch.object(module.shutil, "which", return_value="cargo"), patch.object(module.subprocess, "run", side_effect=inspect):
            self.assertEqual(module.verificar_soluciones(), 0)
        self.assertEqual({p: p.read_bytes() for p in before}, before)

    def test_grader_rejects_an_emptied_or_ignored_test_file(self):
        module = load("practica")
        salidas = [
            "running 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out",
            "running 3 tests\n\ntest result: ok. 0 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out",
            "",
        ]
        for salida in salidas:
            completed = subprocess.CompletedProcess([], 0, stdout=salida, stderr="")
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(module.mostrar(completed), 1, salida)

    def test_grader_only_accepts_real_passes(self):
        module = load("practica")
        correcto = subprocess.CompletedProcess([], 0, stdout="test result: ok. 3 passed; 0 failed; 0 ignored", stderr="")
        fallido = subprocess.CompletedProcess([], 101, stdout="test result: FAILED. 1 passed; 2 failed; 0 ignored", stderr="")
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(module.mostrar(correcto), 0)
            self.assertEqual(module.mostrar(fallido), 101)

    @unittest.skipUnless(shutil.which("git") and (ROOT / ".git").exists(), "git no disponible")
    def test_pristine_tests_are_restored(self):
        module = load("practica")
        with tempfile.TemporaryDirectory(prefix="practica-pruebas-") as temporary:
            copy = Path(temporary) / "ejercicios"
            shutil.copytree(module.EXERCISES, copy, ignore=shutil.ignore_patterns("target"))
            (copy / "tests/dia_09.rs").unlink()
            (copy / "tests/dia_10.rs").write_text("#[test]\nfn prueba_falsa() {}\n", encoding="utf-8")
            self.assertTrue(module.restaurar_pruebas(copy))
            self.assertIn("devolver_texto", (copy / "tests/dia_09.rs").read_text(encoding="utf-8"))
            self.assertNotIn("prueba_falsa", (copy / "tests/dia_10.rs").read_text(encoding="utf-8"))

    @unittest.skipUnless(shutil.which("cargo"), "Rust no está instalado")
    def test_grader_rejects_wrong_answer_and_accepts_solution(self):
        with tempfile.TemporaryDirectory(prefix="practica-grader-") as temporary:
            copy = Path(temporary)
            shutil.copytree(ROOT / "ejercicios", copy / "ejercicios", ignore=shutil.ignore_patterns("target"))
            (copy / "scripts").mkdir()
            shutil.copy2(ROOT / "scripts/practica.py", copy / "scripts/practica.py")
            student = copy / "ejercicios/src/dia_09.rs"
            student.write_text("pub fn devolver_texto(texto: String) -> (String, usize) { (texto, 0) }")
            command = [sys.executable, str(copy / "scripts/practica.py"), "comprobar", "9"]
            wrong = subprocess.run(command, capture_output=True, text=True)
            self.assertNotEqual(wrong.returncode, 0)
            self.assertIn("FAILED", wrong.stdout)
            shutil.copyfile(copy / "ejercicios/soluciones/dia_09.rs", student)
            correct = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(correct.returncode, 0, correct.stderr)
            self.assertIn("Ejercicio superado", correct.stdout)

    @unittest.skipUnless(shutil.which("cargo"), "Rust no está instalado")
    def test_grading_does_not_reuse_artifacts_from_the_solutions(self):
        # Regresion: compartir el target entre la copia del alumno y la de soluciones
        # hacia que un ejercicio sin resolver se diera por superado.
        with tempfile.TemporaryDirectory(prefix="practica-target-") as temporary:
            copy = Path(temporary)
            shutil.copytree(ROOT / "ejercicios", copy / "ejercicios", ignore=shutil.ignore_patterns("target"))
            (copy / "scripts").mkdir()
            shutil.copy2(ROOT / "scripts/practica.py", copy / "scripts/practica.py")
            script = [sys.executable, str(copy / "scripts/practica.py")]
            soluciones = subprocess.run([*script, "verificar-soluciones"], capture_output=True, text=True)
            self.assertEqual(soluciones.returncode, 0, soluciones.stderr)
            stub = subprocess.run([*script, "comprobar", "9"], capture_output=True, text=True)
            self.assertNotEqual(stub.returncode, 0, stub.stdout)
            self.assertNotIn("Ejercicio superado", stub.stdout)


if __name__ == "__main__":
    unittest.main()
