

use std::io::Write;
use std::str;
use std::io;

use futures::future;
use futures::Future;
use futures::stream::Stream;

use hyper::Error;
use hyper::{Method, Request};
use hyper::Body;
use hyper::header::ContentLength;
use hyper::header::ContentType;
use hyper::Uri;
use hyper::client::Client;

use tokio_core::reactor::Core;

pub struct HttpClient {
    header: String,
}

impl HttpClient {
    pub fn post() {
        let json = r#"{"library":"hyper"}"#;
        let uri = "http://httpbin.org/post".parse::<Uri>().unwrap();

        let mut req: Request<Body> = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let resp = req.query();
        let body = match resp {
            Some(v) => v,
            None => {
                println!("response error");
                ""
            }
        };

        let bs = req.query().unwrap_or_default().as_bytes();

        let s = str::from_utf8(bs);

        println!("restule: {}", &s.unwrap_or_default());
    }

    pub fn get0() {
        let uri = "http://httpbin.org/ip".parse::<Uri>().unwrap();

        let mut req: Request<Body> = Request::new(Method::Get, uri);
        req.headers_mut().set(ContentType::json());

        let resp = req.query();
        let body = match resp {
            Some(v) => v,
            None => {
                println!("response error");
                return;
            }
        };

        println!("end");
    }

    pub fn get() {
        // Core is the Tokio event loop used for making a non-blocking request
        //    let mut core = Core::new().unwrap();
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        let uri = "http://httpbin.org/ip".parse::<Uri>().unwrap();

        let work = client.get(uri).map(|res| {
            // map 用于包装成功的返回值
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());
            //        println!("Body: {:?}", res.body());

//            let r = res.body().concat2().and_then(move |body| {
//                let v: Value = serde_json::from_slice(&body).map_err(|e| {
//                    io::Error::new(
//                        io::ErrorKind::Other,
//                        e
//                    )
//                })?;
//                println!("current IP address is {}", v["origin"]);
//                Ok(())
//            });

            let r = res.body().for_each(|chunk| {
                let s = str::from_utf8(chunk.as_ref());
                                println!("chunk: {}", s.unwrap_or_default());
                //                s.map_err(From::from)
                io::stdout().write_all(&chunk).map_err(From::from)
            }).poll();

            println!("{:?}", r);
            return "ok";
        });

        let r = core.run(work);

        println!("{:?}", r);
    }

//    pub fn action() {
//        let url = Url::parse("https://api.github.com/user").unwrap();
//        let mut req = Request::new(Method::Get, url);
//        let mime: Mime = "application/vnd.github.v3+json".parse().unwrap();
//        let token = String::from("token {Your_Token_Here}");
//        req.headers_mut().set(UserAgent(String::from("github-rs")));
//        req.headers_mut().set(Accept(vec![qitem(mime)]));
//        req.headers_mut().set(Authorization(token));
//
//        let mut event_loop = Core::new().unwrap();
//        let handle = event_loop.handle();
//        let client = Client::configure()
//            .connector(HttpsConnector::new(4, &handle))
//            .build(&handle);
//        let work = client.request(req)
//            .and_then(|res| {
//                println!("Response: {}", res.status());
//                println!("Headers: \n{}", res.headers());
//
//                res.body().fold(Vec::new(), |mut v, chunk| {
//                    v.extend(&chunk[..]);
//                    future::ok::<_, Error>(v)
//                }).and_then(|chunks| {
//                    let s = String::from_utf8(chunks).unwrap();
//                    future::ok::<_, Error>(s)
//                })
//            });
//        let user = event_loop.run(work).unwrap();
//        println!("We've made it outside the request! \
//              We got back the following from our \
//              request:\n");
//        println!("{}", user);
//    }
}