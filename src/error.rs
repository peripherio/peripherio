use valico::json_schema;

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
