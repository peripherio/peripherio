use rmps;
use serde;
use failure::Error;

use protos::peripherio as protos;
use error::MalformedConfigPairError;

pub fn parse_config_list<'a, T>(confs: T) -> Result<protos::Config, Error>
    where T: Iterator<Item=&'a str>
{
    let mut res = protos::Config::new();
    for conf in confs {
        let mut pair: Vec<_> = conf.split("=").collect();
        if pair.len() == 1 {
            pair.push("");
        }
        if pair.len() != 2 {
            return Err(MalformedConfigPairError { config: conf.to_string() }.into());
        }
        res.mut_config().push(get_config_pair(pair[0], pair[1]));
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
