extern crate clap;
extern crate peripherio;

use clap::{Arg, App, SubCommand};

use peripherio::subcommand;
use std::env;

fn main() {
    let matches = App::new("peripherio client")
                          .version("1.0")
                          .author("coord.e <me@coord-e.com>")
                          .about("Peripheral interface abstruction")
                          .arg(Arg::with_name("hostname")
                               .short("H")
                               .long("host")
                               .value_name("HOST")
                               .help("Sets the peripherio server host")
                               .takes_value(true))
                          .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .value_name("PORT")
                               .help("Sets the peripherio server port")
                               .takes_value(true))
                          .subcommand(SubCommand::with_name("device")
                                      .about("Manage devices")
                                      .subcommand(SubCommand::with_name("ls")
                                                  .about("List devices")))
                          .get_matches();

    let host_env = env::var("PERIPHERIO_HOST");
    let port_env = env::var("PERIPHERIO_PORT");
    let host = matches.value_of("hostname").or(host_env.as_ref().map(|x| &**x).ok()).unwrap_or("localhost");
    let port = matches.value_of("port").or(port_env.as_ref().map(|x| &**x).ok()).unwrap_or("57601");

    if let Some(matches) = matches.subcommand_matches("device") {
        subcommand::device::main(matches);
    }
}
