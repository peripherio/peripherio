use config::{Config, ConfigValue};

use valico::json_schema::schema::{self, CompilationSettings, Schema, ScopedSchema};
use valico::json_schema::{keywords, Scope};

use std::collections::HashMap;

lazy_static! {
    static ref GLOBAL_SCHEMA: HashMap<&'static str, Schema> = vec![
        (
            "if.type",
            json!({
                "type": "string",
                "enum": [
                    "i2c",
                    "spi",
                    "uart"
                ]
            })
        ),
        (
            "if.i2c.busnum",
            json!({
                "type": "integer"
            })
        ),
        (
            "if.i2c.address",
            json!({
                "type": "integer"
            })
        ),
    ].into_iter()
    .map(|(k, v)| (
        k,
        schema::compile(
            v,
            None,
            CompilationSettings::new(&keywords::default(), true)
        ).unwrap()
    )).collect();
}

pub fn validate_config_value(key: &str, value: &ConfigValue) -> bool {
    let scope = Scope::new();
    GLOBAL_SCHEMA
        .get(key)
        .map(|schema| {
            let sschema = ScopedSchema::new(&scope, &schema);
            sschema.validate(value).is_valid()
        }).unwrap_or(true)
}

pub fn validate_config(config: &Config) -> bool {
    config.iter().all(|(k, v)| validate_config_value(k, v))
}
