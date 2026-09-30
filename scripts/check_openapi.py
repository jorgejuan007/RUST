"""Valida OpenAPI y, opcionalmente, respuestas de un servidor temporal aislado."""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import queue
import subprocess
import tempfile
import threading
from urllib.error import HTTPError
from urllib.request import Request, urlopen

from jsonschema import Draft202012Validator
from openapi_spec_validator import validate

ROOT = Path(__file__).resolve().parents[1]
SPEC = ROOT / "src/api/openapi.json"


def document() -> dict:
    return json.loads(SPEC.read_text(encoding="utf-8"))


def validate_value(spec: dict, schema: dict, value) -> None:
    Draft202012Validator({**schema, "components": spec["components"]}).validate(value)


def validate_document(spec: dict) -> None:
    validate(spec)
    for name, schema in spec["components"]["schemas"].items():
        for example in schema.get("examples", []):
            validate_value(spec, {"$ref": f"#/components/schemas/{name}"}, example)


def validate_response(spec: dict, method: str, path: str, status: int, headers, raw: bytes) -> None:
    operation = spec["paths"][path].get(method.lower())
    if operation is None and status == 405:
        response = spec["components"]["responses"]["Error405"]
    else:
        response = operation["responses"][str(status)]
    if "$ref" in response:
        response = spec["components"]["responses"][response["$ref"].rsplit("/", 1)[1]]
    if "content" not in response:
        if raw:
            raise ValueError(f"{method} {path}: se esperaba una respuesta sin cuerpo")
        return
    if headers.get_content_type() != "application/json":
        raise ValueError(f"{method} {path}: Content-Type no es application/json")
    validate_value(spec, response["content"]["application/json"]["schema"], json.loads(raw))


def check_server(spec: dict, binary: Path) -> int:
    # --bind :0 deja que el sistema asigne un puerto libre; no toca datos del alumno.
    with tempfile.TemporaryDirectory(prefix="rust-api-contrato-") as temporary:
        process = subprocess.Popen(
            [str(binary.resolve()), "--bind", "127.0.0.1:0", "--db", str(Path(temporary) / "tasks.sqlite3")],
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        )
        try:
            lines = queue.Queue()
            threading.Thread(target=lambda: lines.put(process.stdout.readline()), daemon=True).start()
            try:
                first = lines.get(timeout=20)
            except queue.Empty as error:
                raise RuntimeError("el servidor no anunció su dirección en 20 segundos") from error
            if not first.startswith("API: http://127.0.0.1:"):
                raise RuntimeError(f"el servidor no arrancó: {first!r}")
            base = first.removeprefix("API: ").strip()
            checks = 0

            def request(method, uri, expected, value=None, raw=None, content_type="application/json"):
                nonlocal checks
                data = raw if raw is not None else (json.dumps(value).encode() if value is not None else None)
                headers = {"Content-Type": content_type} if content_type else {}
                message = Request(base + uri, data=data, headers=headers, method=method)
                try:
                    result = urlopen(message, timeout=5)
                except HTTPError as error:
                    result = error
                with result:
                    payload = result.read()
                    status = result.code
                    if status != expected:
                        raise ValueError(f"{method} {uri}: se esperaba {expected}, recibido {status}: {payload[:200]!r}")
                    path = uri.split("?", 1)[0]
                    if path.startswith("/tasks/"):
                        path = "/tasks/{id}"
                    validate_response(spec, method, path, status, result.headers, payload)
                checks += 1
                return json.loads(payload) if payload else None

            served = request("GET", "/openapi.json", 200)
            if served != spec:
                raise ValueError("el contrato servido no coincide con el del repositorio")
            request("GET", "/salud", 200)
            request("GET", "/stats", 200)
            request("GET", "/tasks", 200)
            task = request("POST", "/tasks", 201, {"titulo": "  ÁRBOL 🌳  "})
            uri = f"/tasks/{task['id']}"
            request("GET", uri, 200)
            request("PATCH", uri, 200, {"titulo": None, "hecha": True})
            request("PATCH", uri, 200, {"titulo": "Actualizada", "hecha": False})
            page = request("GET", "/tasks?hecha=false&q=actual&limit=1&offset=0", 200)
            if page["total"] != 1 or page["tareas"][0]["id"] != task["id"]:
                raise ValueError("los filtros no devolvieron la tarea esperada")
            request("POST", "/tasks", 400, {"titulo": " "})
            request("POST", "/tasks", 400, {"titulo": "\0NUL"})
            request("POST", "/tasks", 400, raw=b"{")
            request("POST", "/tasks", 422, {"titulo": 3})
            request("POST", "/tasks", 422, {"titulo": "X", "extra": 1})
            request("POST", "/tasks", 415, raw=b"{}", content_type=None)
            request("POST", "/tasks", 413, {"titulo": "x" * (2 * 1024 * 1024)})
            request("PATCH", uri, 400, {"titulo": None, "hecha": None})
            request("GET", "/tasks?limit=0", 400)
            request("GET", "/tasks/no-numero", 400)
            request("GET", "/tasks/0", 400)
            request("PUT", "/tasks", 405)
            request("DELETE", uri, 204)
            request("GET", uri, 404)
            request("PATCH", uri, 404, {"hecha": True})
            request("DELETE", uri, 404)
            return checks
        finally:
            process.terminate()
            try:
                process.communicate(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.communicate(timeout=5)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--server", type=Path, help="binario del servidor para probar HTTP en una base temporal")
    args = parser.parse_args()
    spec = document()
    validate_document(spec)
    print("OpenAPI: contrato y ejemplos válidos.", flush=True)
    if args.server:
        checks = check_server(spec, args.server)
        print(f"HTTP: {checks} respuestas reales cumplen el contrato.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
