use std::io;
use std::str;
use bytes::BytesMut;
use std::sync::mpsc;
use std::sync::mpsc::*;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};

use tokio_io::codec::{Encoder, Decoder};

use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_io::codec::length_delimited;

use tokio_service::{Service, NewService};
use futures::{future, Future};

use tokio_proto::TcpServer;

use super::length_base;

pub struct FrameCodec {
    decoder : length_base::Decoder,
}

impl FrameCodec {
    fn new() -> FrameCodec {
//        let mut _builder = length_base::Builder::new()
//            .length_field_offset(5)
//            .length_field_length(4)
//            .length_adjustment(0)
//            .num_skip(9)
//            .max_frame_length(1024 * 1024 * 10)
//            .little_endian();

        let mut _decoder = length_base::Decoder{
            builder: length_base::Builder::new(),
            state: length_base::DecodeState::Head,
        };

        FrameCodec{
            decoder: _decoder,
        }
    }
}

impl Decoder for FrameCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        self.decoder.decode(buf)
    }
}

impl Encoder for FrameCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

pub struct FrameProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for FrameProto {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = BytesMut;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = String;

    // A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, FrameCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(FrameCodec::new()))
    }
}

pub struct FrameService {
    pub sender : Arc<SyncSender<BytesMut>>,
}

impl FrameService {
    pub fn new(_sender : Arc<SyncSender<BytesMut>>) -> FrameService {
        FrameService{
            sender: _sender,
        }
    }
}

impl Service for FrameService {
    // These types must match the corresponding protocol types:
    type Request = BytesMut;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let ss = str::from_utf8(&req).unwrap().to_string();
        println!("request: {} -> ", req.len());
//        println!("request: {}", ss);

        {
            use std::fs::File;
            use std::io::prelude::*;

            use std::fs::OpenOptions;

            let mut options = OpenOptions::new();
            options.write(true);
            options.append(true);

            let mut file = options.open("foo.txt").unwrap();
            file.metadata().unwrap().permissions().set_readonly(false);
            file.write(ss.as_ref()).unwrap();
            file.write(b"\n").unwrap();
            file.flush();
        }

        self.sender.send(req);

        println!("send: ...");
        // In this case, the response is immediate.
        Box::new(future::ok("ok".to_string()))
    }
}

pub struct FrameNewService {
    pub sender : Arc<SyncSender<BytesMut>>,
}

//impl FrameNewService {
//    pub fn new(_sender : Sender<BytesMut>) -> FrameNewService {
//        let clone = Arc::new(_sender);
//        FrameNewService {
//            sender : clone,
//        }
//    }
//}

impl NewService for FrameNewService {
    type Request = BytesMut;
    type Response = String;
    type Error = io::Error;
    type Instance = FrameService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(FrameService{
            sender: self.sender.clone()
        })
    }
}


pub fn serve_frame() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    let proto = FrameProto;// { codec_builder: builder };

    // The builder requires a protocol and an address
    let server = TcpServer::new(proto, addr);

    let (tx, rx) = mpsc::sync_channel(1000);

    let pair: Arc<SyncSender<BytesMut>> = Arc::new(tx);

    let t2 = thread::spawn(move || {
        rx.iter().for_each(move |data |{
            let ss = str::from_utf8(&data).unwrap().to_string();
            println!("receive: {} -> ", data.len());
        });
    });

    let frame_new_service = FrameNewService {
        sender : pair,
    };

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve( frame_new_service);
}

#[cfg(test)]
mod tests {
    use pretty_env_logger;
    use super::*;

    #[test]
    pub fn serve_frame_test() {
        drop(pretty_env_logger::init());
        serve_frame();
    }
}