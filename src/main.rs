//#![deny(warnings)]

extern crate bytes;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate futures;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

extern crate tokio_core;
#[macro_use]
extern crate tokio_io;
extern crate tokio_timer;

extern crate hyper;

extern crate url;

mod common;
mod utils;


fn main() {
    drop(pretty_env_logger::init());

//    utils::http::get();
//    utils::json::parse(r#"{"a":"xx"}"#);
//    println!("main end!");

    common::queue::sync_test();

}
