extern crate clap;
extern crate peripherio;
extern crate grpcio;

use grpcio::{ChannelBuilder, EnvBuilder};
use clap::{Arg, App, SubCommand};

use peripherio::protos::peripherio_grpc::PeripherioClient;
use peripherio::subcommand;

use std::env;
use std::sync::Arc;


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
                          .subcommand(SubCommand::with_name("driver")
                                      .about("Manage drivers")
                                      .arg(Arg::with_name("config")
                                           .help("The Key-Value config pair to use")
                                           .takes_value(true)
                                           .short("c")
                                           .long("config")
                                           .multiple(true)
                                           .number_of_values(1)
                                           )
                                      .subcommand(SubCommand::with_name("ls")
                                                  .about("List drivers")))
                          .subcommand(SubCommand::with_name("device")
                                      .about("Manage devices")
                                      .arg(Arg::with_name("config")
                                           .help("The Key-Value config pair to use")
                                           .takes_value(true)
                                           .short("c")
                                           .long("config")
                                           .multiple(true)
                                           .number_of_values(1)
                                           )
                                      .subcommand(SubCommand::with_name("ls")
                                                  .about("List devices")))
                          .get_matches();

    let host_env = env::var("PERIPHERIO_HOST");
    let port_env = env::var("PERIPHERIO_PORT");
    let host = matches.value_of("hostname").or(host_env.as_ref().map(|x| &**x).ok()).unwrap_or("localhost");
    let port = matches.value_of("port").or(port_env.as_ref().map(|x| &**x).ok()).unwrap_or("57601");

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&format!("{}:{}", host, port));
    let client = PeripherioClient::new(ch);

    if let Some(matches) = matches.subcommand_matches("device") {
        subcommand::device::main(&client, matches).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("driver") {
        subcommand::driver::main(&client, matches).unwrap();
    } else {
        println!("{}", matches.usage());
    }
}
