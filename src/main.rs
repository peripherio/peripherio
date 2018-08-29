#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate futures;
extern crate grpcio;
extern crate rami;
extern crate rmp_serde as rmps;
extern crate serde_json;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use rami::device::device::{self, DeviceManager};
use rami::driver::driver::Driver;
use rami::driver::driver_manager::DriverManager;
use rami::driver::driver_spec::DriverSpec;
use rami::protos::main::*;
use rami::protos::main_grpc::{self, Rami};

#[derive(Clone)]
struct RamiService {
    manager: Arc<Mutex<DeviceManager>>,
}

impl RamiService {
    fn find_with_spec(
        &self,
        p_config: &Config,
        p_spec: Option<&DriverSpecification>,
    ) -> FindResponse {
        let config: HashMap<String, serde_json::value::Value> = p_config
            .get_config()
            .iter()
            .map(|pair| {
                (
                    pair.get_key().to_string(),
                    rmps::from_slice(&pair.get_value()[..]).unwrap(),
                )
            }).collect();
        let spec = if let Some(p) = p_spec {
            let empty_or = |v| {
                if v == "" {
                    None
                } else {
                    Some(v)
                }
            };
            let vendor = p.get_vendor().to_string();
            let category = p.get_category().to_string();
            let name = p.get_name().to_string();
            DriverSpec::new(empty_or(vendor), empty_or(category), empty_or(name))
        } else {
            DriverSpec::new(None, None, None)
        };
        let manager = self.manager.clone();
        let mut manager = manager.lock().unwrap();
        let drivers: Vec<Driver> = manager.driver_manager().suitable_drivers(&spec, &config);
        let devices = manager.detect(config, Some(&drivers)).unwrap();

        let mut resp = FindResponse::new();
        for device in devices {
            let mut res = FindResponse_DetectResult::new();
            let mut p_id = DeviceID::new();
            p_id.set_id(device.id() as u64);
            res.set_id(p_id);
            res.set_display_name(manager.get_device_name(&device).unwrap().clone());
            let config = manager.get_device_config(&device).unwrap();
            let mut p_config = Config::new();
            for (k, v) in config {
                let mut pair = Config_Pair::new();
                pair.set_key(k.clone());
                pair.set_value(rmps::to_vec(v).unwrap());
                p_config.mut_config().push(pair);
            }
            res.set_config(p_config);
            resp.mut_results().push(res);
        }
        resp
    }
}

impl Rami for RamiService {
    fn list(&self, ctx: RpcContext, req: Config, sink: UnarySink<FindResponse>) {
        let resp = self.find_with_spec(&req, None);
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn find(&self, ctx: RpcContext, req: FindRequest, sink: UnarySink<FindResponse>) {
        let resp = self.find_with_spec(req.get_config(), Some(req.get_spec()));
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

    fn dispatch(&self, ctx: RpcContext, req: DispatchRequest, sink: UnarySink<DispatchResponse>) {
        let resp = {
            let device_id = req.get_device();
            let device = device::Device::with_id(device_id.get_id() as usize);
            let command = req.get_command();
            let args: HashMap<String, serde_json::Value> = rmps::from_slice(&req.get_args()[..]).unwrap();

            let manager = self.manager.clone();
            let mut manager = manager.lock().unwrap();

            let driver = manager.get_device_driver(&device).unwrap();
            let config = manager.get_device_config(&device).unwrap();

            let driver_data = manager.driver_manager().get_data(driver).unwrap();
            let return_data = driver_data.dispatch(command, &args, config).unwrap();

            let mut resp = DispatchResponse::new();
            resp.set_rets(rmps::to_vec(&return_data).unwrap());
            resp
        };
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let mut manager = DeviceManager::new().unwrap();
    let service = main_grpc::create_rami(RamiService {
        manager: Arc::new(Mutex::new(manager)),
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
