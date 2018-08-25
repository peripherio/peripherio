use valico::json_schema;
use serde_json::value::{Value, Number};

#[derive(Fail, Debug)]
#[fail(display = "Cannot represent value \"{}\" in JSON format", value)]
pub struct InvalidNumberError {
    pub value: f64
}

#[derive(Fail, Debug)]
#[fail(display = "Cannot convert JSON value \"{}\" to number", value)]
pub struct InvalidJSONNumberError {
    pub value: Number
}

#[derive(Fail, Debug)]
#[fail(display = "Cannot resolve name {}", name)]
pub struct CannotResolveError {
    pub name: String
}

#[derive(Fail, Debug)]
#[fail(display = "Invalid Schema: {:?}", error)]
pub struct SchemaError {
    pub error: json_schema::SchemaError
}

impl From<json_schema::SchemaError> for SchemaError {
    fn from(error: json_schema::SchemaError) -> Self {
        SchemaError { error }
    }
}
