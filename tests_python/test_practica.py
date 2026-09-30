from pathlib import Path
import os
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
        self.assertEqual([lesson["dia"] for lesson in catalog], [9, 10, 11, 12, 16, 17, 18, 19, 20])
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
            return subprocess.CompletedProcess(arguments, 0)

        with patch.object(module.shutil, "which", return_value="cargo"), patch.object(module.subprocess, "run", side_effect=inspect):
            self.assertEqual(module.verify_solutions(), 0)
        self.assertEqual({p: p.read_bytes() for p in before}, before)

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
            environment = os.environ.copy()
            environment["CARGO_TARGET_DIR"] = str(copy / "target")
            wrong = subprocess.run(command, capture_output=True, text=True, env=environment)
            self.assertNotEqual(wrong.returncode, 0)
            self.assertIn("FAILED", wrong.stdout)
            shutil.copyfile(copy / "ejercicios/soluciones/dia_09.rs", student)
            correct = subprocess.run(command, capture_output=True, text=True, env=environment)
            self.assertEqual(correct.returncode, 0, correct.stderr)
            self.assertIn("Ejercicio superado", correct.stdout)


if __name__ == "__main__":
    unittest.main()
