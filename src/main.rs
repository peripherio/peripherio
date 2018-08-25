#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate futures;
extern crate grpcio;
extern crate rami;
extern crate serde_json;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use rami::device::driver::Driver;
use rami::device::driver_spec::DriverSpec;
use rami::device::driver_manager::DriverManager;
use rami::protos::main::*;
use rami::protos::main_grpc::{self, Rami};

#[derive(Clone)]
struct RamiService {
    manager: Arc<DriverManager>,
}

impl Rami for RamiService {
    fn list(&self, ctx: RpcContext, req: Config, sink: UnarySink<FindResponse>) {
        let config: HashMap<String, serde_json::value::Value> = req
            .get_config()
            .iter()
            .map(|pair| {
                (
                    pair.get_key().to_string(),
                    serde_json::from_str(pair.get_value()).unwrap(),
                )
            })
            .collect();
        let manager = self.manager.clone();
        let spec = DriverSpec::new(None, None, None);
        let drivers: Vec<&Driver> = manager.suitable_drivers(&spec, &config).collect();

        let device = Device::new();
        let mut resp = FindResponse::new();
        resp.mut_results().push(device);
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn find(&self, ctx: RpcContext, req: FindRequest, sink: UnarySink<FindResponse>) {
        let device = Device::new();
        let mut resp = FindResponse::new();
        resp.mut_results().push(device);
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn ping_device(&self, ctx: RpcContext, req: PingRequest, sink: UnarySink<PingResponse>) {
        let mut resp = PingResponse::new();
        resp.set_alive(true);
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let mut manager = DriverManager::new();
    if let Err(e) = manager.load_all() {
        eprintln!("Error: {:?}", e);
        return;
    }
    let service = main_grpc::create_rami(RamiService {
        manager: Arc::new(manager),
    });
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        let stdout = io::stdout();
        let _ = writeln!(&mut stdout.lock(), "Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
