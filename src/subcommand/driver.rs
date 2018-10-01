use clap::ArgMatches;
use failure::Error;
use serde_json;

use protos::peripherio::{Config, DriverSpecification, FindRequest};
use protos::peripherio_grpc::PeripherioClient;
use subcommand::util;

pub fn main(client: &PeripherioClient, matches: &ArgMatches) -> Result<(), Error> {
    Ok(if let Some(matches) = matches.subcommand_matches("ls") {
        list(client, matches)?
    } else {
        println!("{}", matches.usage())
    })
}

pub fn list(client: &PeripherioClient, matches: &ArgMatches) -> Result<(), Error> {
    let conf: Config = matches
        .values_of("config")
        .map(|confs| util::parse_config_list(confs))
        .unwrap_or(Ok(Config::new()))?;
    let mut req = FindRequest::new();
    req.set_config(conf);
    let spec = util::get_driver_spec_from_matches(matches);
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
