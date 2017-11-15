//#![deny(warnings)]

extern crate bytes;
extern crate num_cpus;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate futures;
extern crate futures_cpupool;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

extern crate tokio_core;
#[macro_use]
extern crate tokio_io;
extern crate tokio_service;
extern crate tokio_proto;
extern crate tokio_timer;

extern crate hyper;

extern crate url;

mod common;
mod utils;
mod net;

fn main() {
    drop(pretty_env_logger::init());

//    utils::http::get();
//    utils::json::parse(r#"{"a":"xx"}"#);
//    println!("main end!");

//    common::queue::sync_test();

}
