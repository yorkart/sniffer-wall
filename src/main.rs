//#![deny(warnings)]

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

mod config;
use config::reader;
mod utils;
use utils::http_client;


fn main() {
    reader::t();

//    http_client::HttpClient::post();
    http_client::HttpClient::get();
    println!("main end!");
}

//fn get_content(url: &str) {
//    let url = match url.parse::<Uri>() {
//        Ok(url) => url,
//        Err(_) => return Err(UriError),
//    };
//    let fresh_request = Request::get(url)?;
//    let streaming_request = fresh_request.start()?;
//    let mut response = streaming_request.send()?;
//    Ok(response.read_to_string()?)
//}