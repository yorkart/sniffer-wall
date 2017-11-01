use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

use serde_json::{self, Value};

pub fn get() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = "http://httpbin.org/ip".parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        info!("Response: {}", res.status());

        let body = res.body();
        warn!("body type: {:?}", body);

        body.for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });

    core.run(work).unwrap();
}

pub fn get_json(address : &str) -> Result<&str, io::Error> {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    //    let uri = "http://httpbin.org/ip".parse().unwrap();
    let uri = address.parse().map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;
    let work = client.get(uri).and_then(|res| {
        info!("Response: {}", res.status());

        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, e)
            })?;
            warn!("current IP address is {}", v["origin"]);
            Ok(v)
        })
    });

    let b= core.run(work).unwrap();
    info!("xx {:?}", b);

    return Ok("Yes".trim());
}