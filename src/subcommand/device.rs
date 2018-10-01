use clap::ArgMatches;
use failure::Error;
use rmps;
use serde_json;

use error::MalformedConfigPairError;
use protos::peripherio::{Config, DriverSpecification, FindRequest};
use protos::peripherio_grpc::PeripherioClient;
use subcommand::util;

pub fn main(client: &PeripherioClient, matches: &ArgMatches) -> Result<(), Error> {
    let req: Config = matches
        .values_of("config")
        .map(|confs| util::parse_config_list(confs))
        .unwrap_or(Ok(Config::new()))?;
    let spec = util::get_driver_spec_from_matches(matches);
    Ok(if let Some(matches) = matches.subcommand_matches("ls") {
        list(client, matches, req, spec)?
    } else {
        println!("{}", matches.usage())
    })
}

pub fn list(client: &PeripherioClient, matches: &ArgMatches, conf: Config, spec: DriverSpecification) -> Result<(), Error> {
    let mut req = FindRequest::new();
    req.set_config(conf);
    req.set_spec(spec);
    let reply = client.find(&req)?;
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
