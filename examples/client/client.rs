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

use grpcio::{ChannelBuilder, EnvBuilder};
use rami::protos::main::*;
use rami::protos::main_grpc::RamiClient;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = RamiClient::new(ch);

    let mut req = Config::new();
    let reply = client.list(&req).expect("rpc");
    println!("Greeter received: {:?}", reply.get_results());
}
