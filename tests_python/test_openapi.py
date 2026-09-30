from email.message import Message
import unittest
from jsonschema import ValidationError
from test_documentacion import load


class OpenApiTests(unittest.TestCase):
    def setUp(self):
        self.module = load("check_openapi")
        self.spec = self.module.document()

    def test_contract_and_all_examples_are_valid(self):
        self.module.validate_document(self.spec)

    def test_task_and_update_reject_incorrect_types_and_empty_changes(self):
        for schema, value in [
            ("Task", {"id": 0, "titulo": "X", "hecha": False}),
            ("Task", {"id": 1, "titulo": "X", "hecha": "false"}),
            ("Task", {"id": 1, "titulo": "X", "hecha": False, "extra": 1}),
            ("UpdateTask", {}),
            ("UpdateTask", {"titulo": None, "hecha": None}),
            ("UpdateTask", {"hecha": 1}),
        ]:
            with self.subTest(schema=schema, value=value), self.assertRaises(ValidationError):
                self.module.validate_value(self.spec, {"$ref": f"#/components/schemas/{schema}"}, value)
        self.module.validate_value(self.spec, {"$ref": "#/components/schemas/UpdateTask"}, {"hecha": False, "titulo": None})

    def test_checker_rejects_wrong_media_type_extra_fields_and_delete_body(self):
        headers = Message()
        headers["Content-Type"] = "application/json"
        with self.assertRaises(ValidationError):
            self.module.validate_response(self.spec, "GET", "/stats", 200, headers, b'{"total":0}')
        with self.assertRaises(ValueError):
            self.module.validate_response(self.spec, "DELETE", "/tasks/{id}", 204, headers, b'{}')
        headers.replace_header("Content-Type", "text/html")
        with self.assertRaises(ValueError):
            self.module.validate_response(self.spec, "GET", "/stats", 200, headers, b'{"total":0,"pendientes":0}')


if __name__ == "__main__":
    unittest.main()
