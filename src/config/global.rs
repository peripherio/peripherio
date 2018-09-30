use config::{Config, ConfigValue};

use valico::json_schema::Scope;

use std::collections::HashMap;

lazy_static! {
    pub static ref GLOBAL_SCHEMA: HashMap<String, ConfigValue> = vec![
        (
            "if.type".to_string(),
            json!({
                "type": "string",
                "enum": [
                    "i2c",
                    "spi",
                    "uart"
                ]
            }),
        ),
        (
            "if.i2c.busnum".to_string(),
            json!({
                "type": "integer"
            }),
        ),
        (
            "if.i2c.address".to_string(),
            json!({
                "type": "integer"
            }),
        ),
    ].into_iter()
        .collect();
}

pub fn validate_config_value(key: &str, value: &ConfigValue) -> bool {
    let mut scope = Scope::new();
    GLOBAL_SCHEMA
        .get(key)
        .map(|schema_data| {
            let sschema = scope
                .compile_and_return(schema_data.clone(), true)
                .ok()
                .unwrap();
            sschema.validate(value).is_valid()
        })
        .unwrap_or(true)
}

pub fn validate_config(config: &Config) -> bool {
    config.iter().all(|(k, v)| validate_config_value(k, v))
}
