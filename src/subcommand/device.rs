use failure::Error;
use clap::ArgMatches;
use rmps;
use serde_json;

use protos::peripherio_grpc::PeripherioClient;
use protos::peripherio::Config;
use subcommand::util;
use error::MalformedConfigPairError;

pub fn main(client: &PeripherioClient, matches: &ArgMatches) -> Result<(), Error> {
    let mut req = Config::new();
    if let Some(confs) = matches.values_of("config") {
        req = util::parse_config_list(confs)?;
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

