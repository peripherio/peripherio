pub mod global;

use serde_json::value::Value;
use std::collections::HashMap;

pub type ConfigValue = Value;
pub type Config = HashMap<String, ConfigValue>;
