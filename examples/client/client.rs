// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate grpcio;
extern crate rami;

use std::sync::Arc;
use std::time::{Duration, Instant};

use grpcio::{ChannelBuilder, EnvBuilder};
use rami::protos::main::*;
use rami::protos::main_grpc::RamiClient;

fn get_pair(k: &str, v: &str) -> Config_Pair {
    let mut pair = Config_Pair::new();
    pair.set_key(k.to_string());
    pair.set_value(v.to_string());
    pair
}

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = RamiClient::new(ch);

    let start = Instant::now();
    let mut req = Config::new();
    req.mut_config().push(get_pair("if.type", "\"i2c\""));
    req.mut_config().push(get_pair("if.i2c.speed", "100"));
    // req.mut_config().push(get_pair("if.i2c.busnum", "0"));
    // req.mut_config().push(get_pair("if.i2c.address", "0"));
    let reply = client.list(&req).expect("rpc");
    let end = start.elapsed();
    println!(
        "Elapsed time: {}ns({}ms)",
        end.subsec_nanos(),
        end.subsec_nanos() as f64 / 1000000.0
    );
    println!("Received: {:?}", reply.get_results());
}
