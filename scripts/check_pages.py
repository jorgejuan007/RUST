"""Verifica que Pages publique el artefacto de la revisión CI actual."""
import json
import os
import re
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen


def validate_run(run: dict, repository: str, latest_sha: str) -> None:
    conditions = [
        run.get("path") == ".github/workflows/ci.yml",
        run.get("event") == "push",
        run.get("status") == "completed",
        run.get("conclusion") == "success",
        run.get("head_branch") in {"main", "master"},
        (run.get("head_repository") or {}).get("full_name") == repository,
        run.get("head_sha") == latest_sha,
    ]
    if not all(conditions):
        raise ValueError("La ejecución debe ser un CI exitoso de un push a la revisión actual de main/master del mismo repositorio.")


def main() -> None:
    repository = os.environ.get("GITHUB_REPOSITORY", "")
    run_id = os.environ.get("PAGES_RUN_ID", "")
    token = os.environ.get("GH_TOKEN", "")
    if not re.fullmatch(r"[\w.-]+/[\w.-]+", repository) or not run_id.isascii() or not run_id.isdigit() or not token:
        raise ValueError("Faltan el repositorio, un identificador CI numérico o el token de Actions.")

    def get(resource: str) -> dict:
        request = Request(f"https://api.github.com/repos/{repository}/{resource}", headers={"Authorization": f"Bearer {token}", "Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"})
        with urlopen(request, timeout=30) as response:
            return json.load(response)

    run = get(f"actions/runs/{run_id}")
    branch = run.get("head_branch")
    if branch not in {"main", "master"}:
        raise ValueError("Pages solo publica main o master.")
    latest_sha = get(f"git/ref/heads/{branch}")["object"]["sha"]
    validate_run(run, repository, latest_sha)
    try:
        pages = get("pages")
    except HTTPError as error:
        if error.code in {403, 404}:
            raise ValueError("Pages no está habilitado o no es accesible. El propietario debe seleccionar Settings → Pages → Source → GitHub Actions.") from error
        raise
    if pages.get("build_type") != "workflow":
        raise ValueError("Configura Settings → Pages → Source → GitHub Actions.")
    with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
        output.write(f"run_id={run_id}\n")
    print(f"CI {run_id} validado para {run['head_sha']}; Pages preparado.")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, HTTPError) as error:
        raise SystemExit(str(error))
