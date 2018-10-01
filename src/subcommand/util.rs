use failure::Error;
use rmps;
use serde;

use error::MalformedConfigPairError;
use protos::peripherio as protos;

pub fn parse_config_list<'a, T>(confs: T) -> Result<protos::Config, Error>
where
    T: Iterator<Item = &'a str>,
{
    let mut res = protos::Config::new();
    for conf in confs {
        let mut pair: Vec<_> = conf.split("=").collect();
        if pair.len() == 1 {
            pair.push("");
        }
        if pair.len() != 2 {
            return Err(MalformedConfigPairError {
                config: conf.to_string(),
            }.into());
        }
        let config_pair = if let Ok(num) = pair[1].parse::<i64>() {
            get_config_pair(pair[0], &num)
        } else if let Ok(num) = pair[1].parse::<f64>() {
            get_config_pair(pair[0], &num)
        } else {
            get_config_pair(pair[0], pair[1])
        };
        res.mut_config().push(config_pair);
    }
    Ok(res)
}

pub fn get_config_pair<T: ?Sized>(k: &str, v: &T) -> protos::Config_Pair
where
    T: serde::Serialize,
{
    let mut pair = protos::Config_Pair::new();
    pair.set_key(k.to_string());
    pair.set_value(rmps::to_vec(v).unwrap());
    pair
}
