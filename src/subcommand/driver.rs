use failure::Error;
use clap::ArgMatches;
use serde_json;

use protos::peripherio_grpc::PeripherioClient;
use protos::peripherio::{Config, DriverSpecification, FindRequest};
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

pub fn list(client: &PeripherioClient, matches: &ArgMatches, conf: &Config) -> Result<(), Error> {
    let mut req = FindRequest::new();
    req.set_config(conf.clone());
    let spec = DriverSpecification::new();
    req.set_spec(spec);
    let reply = client.find_drivers(&req)?;
    println!("NAME VENDOR PATH CATEGORIES");
    for res in reply.get_results() {
        let name = res.get_name();
        let vendor = res.get_vendor();
        let path = res.get_path();
        let categories = res.get_category();
        println!("{} {} {} {}", name, vendor, path, categories.join(","));
    }
    Ok(())
}
