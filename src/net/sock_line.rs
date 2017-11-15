use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};

use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_io::codec::length_delimited;

use tokio_service::Service;
use futures::{future, Future};

use tokio_proto::TcpServer;

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = String;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = String;

    // A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

pub struct LineProto2 {
    codec_builder: length_delimited::Builder,
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto2 {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = BytesMut;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = BytesMut;

    // A bit of boilerplate to hook in the codec:
    type Transport = length_delimited::Framed<T>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let framed = self.codec_builder.new_framed(io);
        Ok(framed)
    }
}

pub struct Echo;

impl Service for Echo {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        println!("request: {}", req);
        // In this case, the response is immediate.
        Box::new(future::ok(req))
    }
}


pub struct Echo2;

impl Service for Echo2 {
    // These types must match the corresponding protocol types:
    type Request = BytesMut;
    type Response = BytesMut;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let ss = str::from_utf8(&req).unwrap().to_string();
        println!("request: {}", ss);

        // In this case, the response is immediate.
        Box::new(future::ok(req))
    }
}

pub fn serve_line() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    let mut builder = length_delimited::Builder::new();
    builder
        .length_field_offset(5)
        .length_field_length(4)
        .length_adjustment(0)
        .num_skip(0)
        .max_frame_length(1024 * 1024 * 10)
        .little_endian();
    let proto = LineProto;// { codec_builder: builder };

    // The builder requires a protocol and an address
    let server = TcpServer::new(proto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Echo));
}

#[cfg(test)]
mod tests {
    use pretty_env_logger;
    use super::*;

    #[test]
    pub fn serve_line_test() {
        drop(pretty_env_logger::init());
        serve_line();
    }
}