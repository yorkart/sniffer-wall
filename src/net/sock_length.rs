
use std::io;
use bytes;
//
//use futures::future;
//use futures::{Future, BoxFuture};
//use tokio_io::{AsyncRead, AsyncWrite};
//use tokio_io::codec::length_delimited;
//
//use tokio_proto::TcpServer;
//use tokio_proto::pipeline::ServerProto;
//use tokio_service::Service;
//
//type MsgType = bytes::BytesMut;
//
//struct LineProto {
//    codec_builder: length_delimited::Builder,
//}
//
//impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
//    type Request = MsgType;
//    type Response = MsgType;
//    type Transport = length_delimited::Framed<T>;
//    type BindTransport = Result<Self::Transport, io::Error>;
//
//    fn bind_transport(&self, io: T) -> Self::BindTransport {
//        let framed = self.codec_builder.new_framed(io);
//        Ok(framed)
//    }
//}
//
//struct Echo;
//
//impl Service for Echo {
//    type Request = MsgType;
//    type Response = MsgType;
//    type Error = io::Error;
//    type Future = BoxFuture<Self::Response, Self::Error>;
//
//    fn call(&self, req: Self::Request) -> Self::Future {
//        future::ok(req).boxed()
//    }
//}
//
//pub fn serve_length() {
//    // Specify the localhost address
//    let addr = "10.101.22.31:8998".parse().unwrap();
//
//    // The builder requires a protocol and an address
//    let mut builder = length_delimited::Builder::new();
//    builder
//        .length_field_offset(5)
//        .length_field_length(4)
//        .length_adjustment(0)
//        .num_skip(0)
//        .max_frame_length(1024 * 1024 * 10)
//        .little_endian();
//
//    let proto = LineProto { codec_builder: builder };
//    let server = TcpServer::new(proto, addr);
//
//    // We provide a way to *instantiate* the service for each new
//    // connection; here, we just immediately return a new instance.
//    server.serve(|| Ok(Echo));
//}
//
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    pub fn serve_length_test() {
//        serve_length();
//    }
//}