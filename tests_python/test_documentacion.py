import importlib.util
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


def load(name):
    spec = importlib.util.spec_from_file_location(name, ROOT / "scripts" / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class DocumentationTests(unittest.TestCase):
    def test_all_source_links_exist(self):
        self.assertEqual(load("check_docs").check(), [])

    def test_machine_links_and_missing_targets_are_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "docs").mkdir()
            (root / "ejercicios").mkdir()
            (root / "manual_rust_30_dias.md").write_text("")
            (root / "ejercicios/README.md").write_text("")
            (root / "README.md").write_text("[local](/Users/alguien/curso.md) [file](file:///tmp/curso.md) [falta](no-existe.md) [fuera](../fuera.md)")
            self.assertEqual(len(load("check_docs").check(root)), 4)

    def test_site_is_identical_after_relocation(self):
        with tempfile.TemporaryDirectory(prefix="curso con espacios ") as temporary:
            relocated = Path(temporary) / "RUST"
            relocated.mkdir()
            for name in ["README.md", "manual_rust_30_dias.md"]:
                shutil.copy2(ROOT / name, relocated / name)
            for name in ["docs", "scripts", "src", "data", "tests", "tests_python", ".github", "ejercicios"]:
                shutil.copytree(ROOT / name, relocated / name, ignore=shutil.ignore_patterns("target", "__pycache__"))
            for name in ["Cargo.toml", "Cargo.lock", "Makefile", "mkdocs.yml", "requirements-docs.txt", "rust-toolchain.toml"]:
                shutil.copy2(ROOT / name, relocated / name)
            subprocess.run([sys.executable, str(relocated / "scripts/sync_mkdocs.py")], check=True, cwd=temporary)
            local = {path.relative_to(ROOT / "site_docs"): path.read_bytes() for path in (ROOT / "site_docs").rglob("*") if path.is_file()}
            remote = {path.relative_to(relocated / "site_docs"): path.read_bytes() for path in (relocated / "site_docs").rglob("*") if path.is_file()}
            self.assertEqual(local, remote)
            for name, contents in remote.items():
                if name.suffix == ".md":
                    self.assertNotIn(b"](/Users/", contents)
                    self.assertNotIn(str(relocated).encode(), contents)


class PagesTests(unittest.TestCase):
    def valid_run(self):
        return {"path": ".github/workflows/ci.yml", "event": "push", "status": "completed", "conclusion": "success", "head_branch": "main", "head_repository": {"full_name": "owner/repo"}, "head_sha": "actual"}

    def test_accepts_current_successful_main_run(self):
        load("check_pages").validate_run(self.valid_run(), "owner/repo", "actual")

    def test_rejects_unvalidated_stale_or_foreign_runs(self):
        validator = load("check_pages").validate_run
        for key, invalid in [("event", "pull_request"), ("conclusion", "failure"), ("status", "in_progress"), ("head_branch", "feature"), ("head_repository", {"full_name": "other/repo"}), ("head_sha", "antigua"), ("path", ".github/workflows/otro.yml")]:
            run = self.valid_run()
            run[key] = invalid
            with self.subTest(key=key), self.assertRaises(ValueError):
                validator(run, "owner/repo", "actual")


if __name__ == "__main__":
    unittest.main()
