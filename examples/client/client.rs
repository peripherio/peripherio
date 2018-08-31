extern crate grpcio;
extern crate peripherio;
extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::sync::Arc;
use std::time::{Duration, Instant};

use grpcio::{ChannelBuilder, EnvBuilder};
use peripherio::protos::peripherio::*;
use peripherio::protos::peripherio_grpc::PeripherioClient;

fn get_pair<T: ?Sized>(k: &str, v: &T) -> Config_Pair
where
    T: serde::Serialize,
{
    let mut pair = Config_Pair::new();
    pair.set_key(k.to_string());
    pair.set_value(rmps::to_vec(v).unwrap());
    pair
}

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = PeripherioClient::new(ch);

    let start = Instant::now();
    let mut req = Config::new();
    req.mut_config().push(get_pair("if.type", "i2c"));
    req.mut_config().push(get_pair("if.i2c.speed", &100));
    // req.mut_config().push(get_pair("if.i2c.busnum", "0"));
    // req.mut_config().push(get_pair("if.i2c.address", "0"));
    let reply = client.list(&req).expect("rpc");
    let end = start.elapsed();
    println!(
        "Elapsed time: {}ns({}ms)",
        end.subsec_nanos(),
        end.subsec_nanos() as f64 / 1000000.0
    );
    for res in reply.get_results() {
        let device = res.get_id().get_id();
        let device_name = res.get_display_name();
        let config = res.get_config();
        println!("{} => {}", device, device_name);
        for conf in config.get_config() {
            let value: serde_json::Value = rmps::from_slice(&conf.get_value()[..]).unwrap();
            println!("\t{} => {}", conf.get_key(), value);
        }
    }

    let start = Instant::now();
    let mut req = DispatchRequest::new();
    let mut id = DeviceID::new();
    id.set_id(0);
    req.set_device(id);
    req.set_command("getman".to_string());
    req.set_args(rmps::to_vec(&json!({"value": 10})).unwrap());
    let reply = client.dispatch(&req).expect("rpc");
    let end = start.elapsed();
    println!(
        "Elapsed time: {}ns({}ms)",
        end.subsec_nanos(),
        end.subsec_nanos() as f64 / 1000000.0
    );
    let value: serde_json::Value = rmps::from_slice(&reply.get_rets()[..]).unwrap();
    println!("Received: {:?}", value);
}
