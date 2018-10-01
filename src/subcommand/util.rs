use rmps;
use serde;

use protos::peripherio as protos;


pub fn get_config_pair<T: ?Sized>(k: &str, v: &T) -> protos::Config_Pair
where
    T: serde::Serialize,
{
    let mut pair = protos::Config_Pair::new();
    pair.set_key(k.to_string());
    pair.set_value(rmps::to_vec(v).unwrap());
    pair
}
