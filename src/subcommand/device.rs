use failure::Error;
use clap::ArgMatches;
use rmps;
use serde_json;
use serde;

use protos::peripherio_grpc::PeripherioClient;
use protos::peripherio::{Config, Config_Pair};
use error::MalformedConfigPairError;

fn get_config_pair<T: ?Sized>(k: &str, v: &T) -> Config_Pair
where
    T: serde::Serialize,
{
    let mut pair = Config_Pair::new();
    pair.set_key(k.to_string());
    pair.set_value(rmps::to_vec(v).unwrap());
    pair
}

pub fn main(client: &PeripherioClient, matches: &ArgMatches) -> Result<(), Error> {
    let mut req = Config::new();
    if let Some(confs) = matches.values_of("config") {
        for conf in confs {
            let mut pair: Vec<_> = conf.split("=").collect();
            if pair.len() == 1 {
                pair.push("");
            }
            if pair.len() != 2 {
                return Err(MalformedConfigPairError { config: conf.to_string() }.into());
            }
            req.mut_config().push(get_config_pair(pair[0], pair[1]));
        }
    }
    Ok(if let Some(matches) = matches.subcommand_matches("ls") {
        list(client, matches, &req)?
    } else {
        println!("{}", matches.usage())
    })
}

pub fn list(client: &PeripherioClient, matches: &ArgMatches, req: &Config) -> Result<(), Error> {
    let reply = client.list(req)?;
    for res in reply.get_results() {
        let device = res.get_id().get_id();
        let device_name = res.get_display_name();
        let config = res.get_config();
        println!("{} => {}", device, device_name);
        for conf in config.get_config() {
            let value: serde_json::Value = rmps::from_slice(&conf.get_value()[..])?;
            println!("\t{} => {}", conf.get_key(), value);
        }
    }
    Ok(())
}

