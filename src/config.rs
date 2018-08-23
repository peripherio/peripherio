use valico::json_schema::keywords;
use valico::json_schema::schema::{self, Schema, CompilationSettings};
use serde_json::value::Value;
use std::collections::HashMap;

pub type ConfigValue = Value;
pub type Config = HashMap<String, ConfigValue>;

lazy_static! {
    static ref GLOBAL_SCHEMA: HashMap<&'static str, Schema> = vec![
            ("if.type", json!({
                "type": "string",
                "enum": [
                    "i2c",
                    "spi",
                    "uart"
                ]
            })),
            ("if.i2c.busnum", json!({
                "type": "integer"
            })),
            ("if.i2c.address", json!({
                "type": "integer"
            })),
        ]
        .into_iter().map(|(k, v)| {
            (k, schema::compile(v, None, CompilationSettings::new(&keywords::default(), true)).unwrap())
        })
        .collect();
}
