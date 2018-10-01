pub mod global;

use rmps;
use serde_json::value::Value;
use std::collections::HashMap;

use protos::peripherio as protos;

pub type ConfigValue = Value;
pub type Config = HashMap<String, ConfigValue>;

impl<'a> From<&'a protos::Config> for Config {
    fn from(p_config: &'a protos::Config) -> Self {
        p_config
            .get_config()
            .iter()
            .map(|pair| {
                (
                    pair.get_key().to_string(),
                    rmps::from_slice(&pair.get_value()[..]).unwrap(),
                )
            })
            .collect()
    }
}

impl Into<protos::Config> for Config {
    fn into(self) -> protos::Config {
        let mut p_config = protos::Config::new();
        for (k, v) in self {
            let mut pair = protos::Config_Pair::new();
            pair.set_key(k);
            pair.set_value(rmps::to_vec(&v).unwrap());
            p_config.mut_config().push(pair);
        }
        p_config
    }
}
