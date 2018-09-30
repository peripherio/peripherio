use failure::Error;
use clap::ArgMatches;

use protos::peripherio_grpc::PeripherioClient;

pub fn main(client: &PeripherioClient, matches: &ArgMatches) -> Result<(), Error> {
    Ok(())
}

