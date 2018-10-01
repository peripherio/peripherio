#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate ctrlc;
extern crate futures;
extern crate grpcio;
extern crate peripherio;
extern crate rmp_serde as rmps;
extern crate serde_json;

use std::collections::HashMap;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use peripherio::device::{self, DeviceManager};
use peripherio::driver;
use peripherio::config;
use peripherio::protos::peripherio::*;
use peripherio::protos::peripherio_grpc::{self, Peripherio};


#[derive(Clone)]
struct PeripherioService {
    manager: Arc<Mutex<DeviceManager>>,
}

impl PeripherioService {
    fn find_with_spec(
        &self,
        p_config: &Config,
        p_spec: Option<&DriverSpecification>,
    ) -> FindResponse {
        let config = config::Config::from(p_config);
        let spec = driver::DriverSpec::from(p_spec);

        let manager = self.manager.clone();
        let mut manager = manager.lock().unwrap();
        let drivers = manager.driver_manager().suitable_drivers(&spec, &config);

        let devices = manager.detect(config, Some(&drivers)).unwrap();

        let mut resp = FindResponse::new();
        for device in devices {
            let mut res = FindResponse_DetectResult::new();
            let mut p_id = DeviceID::new();
            p_id.set_id(device.id() as u64);
            res.set_id(p_id);
            res.set_display_name(manager.get_device_name(&device).unwrap().clone());
            let config = manager.get_device_config(&device).unwrap();
            res.set_config(config.clone().into());
            resp.mut_results().push(res);
        }
        resp
    }
}

impl Peripherio for PeripherioService {
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
            let args: HashMap<String, serde_json::Value> =
                rmps::from_slice(&req.get_args()[..]).unwrap();

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
    let manager = DeviceManager::new().unwrap();
    let service = peripherio_grpc::create_peripherio(PeripherioService {
        manager: Arc::new(Mutex::new(manager)),
    });
    let host = env::var("PERIPHERIO_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PERIPHERIO_PORT").unwrap_or("57601".to_string());
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind(host, port.parse().unwrap())
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    println!("Ctrl-C to exit...");
    while running.load(Ordering::SeqCst) {}

    let _ = server.shutdown().wait();
}
