use std::env;
use std::net::{self, SocketAddr};
use std::thread;
use std::io::Read;
use std::io::BufReader;

use futures::future;
use futures::Future;
use futures::future::Executor;
use futures::stream::Stream;
use futures::sync::mpsc;
use futures_cpupool::CpuPool;

use tokio_io::io;
use tokio_io::AsyncRead;
use tokio_io::io::copy;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
//use tokio_io::io::{lines, write_all};
use tokio_io::codec::length_delimited::*;

pub fn serve_multi_threads(addr: &str, num_threads: i32) {
    // First argument, the address to bind
    //    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Second argument, the number of threads we'll be using
//    let num_threads = env::args().nth(2).and_then(|s| s.parse().ok())
//        .unwrap_or(num_cpus::get());

    // Use `std::net` to bind the requested port, we'll use this on the main
    // thread below
    let listener = net::TcpListener::bind(&addr).expect("failed to bind");
    println!("Listening on: {}", addr);

    // Spin up our worker threads, creating a channel routing to each worker
    // thread that we'll use below.
    let mut channels = Vec::new();
    for _ in 0..num_threads {
        let (tx, rx) = mpsc::unbounded();
        channels.push(tx);
        thread::spawn(|| worker(rx));
    }

    // Infinitely accept sockets from our `std::net::TcpListener`, as this'll do
    // blocking I/O. Each socket is then shipped round-robin to a particular
    // thread which will associate the socket with the corresponding event loop
    // and process the connection.
    let mut next = 0;
    for socket in listener.incoming() {
        let socket = socket.expect("failed to accept");
        channels[next].unbounded_send(socket).expect("worker thread died");
        next = (next + 1) % channels.len();
    }
}

fn worker(rx: mpsc::UnboundedReceiver<net::TcpStream>) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let done = rx.for_each(move |socket| {
        // First up when we receive a socket we associate it with our event loop
        // using the `TcpStream::from_stream` API. After that the socket is not
        // a `tokio_core::net::TcpStream` meaning it's in nonblocking mode and
        // ready to be used with Tokio
        let socket = TcpStream::from_stream(socket, &handle).expect("failed to associate TCP stream");
        let addr = socket.peer_addr().expect("failed to get remote address");

        // Like the single-threaded `echo` example we split the socket halves
        // and use the `copy` helper to ship bytes back and forth. Afterwards we
        // spawn the task to run concurrently on this thread, and then print out
        // what happened afterwards
//        let (reader, writer) = socket.split();


        let mut builder = Builder::new();
        let frame_read = builder
            .length_field_offset(5)
            .length_field_length(4)
            .length_adjustment(0)
            .num_skip(0)
            .max_frame_length(1024 * 1024 * 10)
            .big_endian()
            .new_read(socket);



//        let a = io_reader.poll().unwrap();
//        a.map(|bytes_mut|  {
//                        let bm = bytes_mut.unwrap();
//                        println!("data: {:?}", &bm[..]);
//        });

//        let msg = io_reader.and_then(|bm| {
//            println!("data: {:?}", &bm[..]);
//            Ok(())
//        }).then(|x| {
//            Ok(())
//        });

//        let msg = io_reader.then(move |bytes_mut | {
//            let bm = bytes_mut.unwrap();
//            println!("data: {:?}", &bm[..]);
//            Ok(())
//        });
//
//        handle.spawn(msg);

        Ok(())
    });
    core.run(done).unwrap();
}

#[cfg(test)]
mod tests {
    use super::serve_multi_threads;

    #[test]
    pub fn serve_multi_threads_test() {
        serve_multi_threads("10.101.22.31:8998", 4);
    }
}