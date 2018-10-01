use serde_json::value::Number;
use valico::json_schema;

#[derive(Fail, Debug)]
#[fail(display = "Internal Error: Use of unknown driver id")]
pub struct DriverNotFoundError;

#[derive(Fail, Debug)]
#[fail(display = "Unknown command: {}", name)]
pub struct UnknownCommandError {
    pub name: String,
}

#[derive(Fail, Debug)]
#[fail(display = "Use of unknown config key: {}", name)]
pub struct UnknownConfigError {
    pub name: String,
}

#[derive(Fail, Debug)]
#[fail(display = "Cannot represent value \"{}\" in JSON format", value)]
pub struct InvalidNumberError {
    pub value: f64,
}

#[derive(Fail, Debug)]
#[fail(display = "Cannot convert JSON value \"{}\" to number", value)]
pub struct InvalidJSONNumberError {
    pub value: Number,
}

#[derive(Fail, Debug)]
#[fail(display = "Cannot find type from schema \"{}\"", field)]
pub struct TypeNotFoundError {
    pub field: String,
}
#[derive(Fail, Debug)]
#[fail(display = "Cannot resolve name {}", name)]
pub struct CannotResolveError {
    pub name: String,
}

#[derive(Fail, Debug)]
#[fail(
    display = "Cannot find all required symbols {:?} and {} in driver {}", requires, common, name
)]
pub struct SymbolsNotEnoughError {
    pub requires: Vec<String>,
    pub common: String,
    pub name: String,
}

#[derive(Fail, Debug)]
#[fail(display = "Invalid Schema: {:?}", error)]
pub struct SchemaError {
    pub error: json_schema::SchemaError,
}

#[derive(Fail, Debug)]
#[fail(display = "Malformed Config Pair: {:?}", config)]
pub struct MalformedConfigPairError {
    pub config: String,
}

impl From<json_schema::SchemaError> for SchemaError {
    fn from(error: json_schema::SchemaError) -> Self {
        SchemaError { error }
    }
}
