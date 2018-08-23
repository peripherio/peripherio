pub mod protos;
pub mod device;
pub mod config;
pub mod error;
pub mod resolve;

extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate serde_yaml;
extern crate valico;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
