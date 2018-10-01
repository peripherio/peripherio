use clap::ArgMatches;
use failure::Error;
use rmps;
use serde_json;

use error::MalformedConfigPairError;
use protos::peripherio::Config;
use protos::peripherio_grpc::PeripherioClient;
use subcommand::util;

pub fn main(client: &PeripherioClient, matches: &ArgMatches) -> Result<(), Error> {
    let req: Config = matches
        .values_of("config")
        .map(|confs| util::parse_config_list(confs))
        .unwrap_or(Ok(Config::new()))?;
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

