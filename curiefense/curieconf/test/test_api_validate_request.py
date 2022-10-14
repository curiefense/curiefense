import pytest
from curieconf.confserver.v2.api import validate_request, schema_type_map, models
from werkzeug.exceptions import NotFound, InternalServerError, BadRequest
from flask import Flask
from unittest.mock import patch

schema_without_errors_messages = {
    "list": {
        "type": "array",
        "title": "list",
        "minItems": 2
    }
}

invalid_schema = {'schema': 'invalid'}

app = Flask(__name__)

@validate_request
def do_nothing(document): pass

def test_validate_request_with_non_existent_document():

    with pytest.raises(NotFound) as err:
        do_nothing(document="non_existent_document")

    assert err.value.code == 404
    assert err.value.description == "document does not exist"

@patch.dict(models, invalid_schema, clear=True)
@patch.dict(schema_type_map, invalid_schema, clear=True)
def test_validate_request_with_invalid_schema():

    with app.test_request_context(json={}), pytest.raises(InternalServerError) as err:
        do_nothing(document="schema")

    assert err.value.code == 500
    assert err.value.description == "'invalid' is not of type 'object', 'boolean'"

@patch.dict(models, schema_without_errors_messages)
@patch.dict(schema_type_map, schema_without_errors_messages)
def test_validate_request_with_non_existent_error_message():

    with app.test_request_context(json=[]), pytest.raises(BadRequest) as err:
        do_nothing(document="list")

    assert err.value.code == 400
    assert err.value.description == "[] is too short"
