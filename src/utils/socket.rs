use std::fmt;
use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr};

use futures::Poll;

use tokio_core::net::TcpStream;
use tokio_io::AsyncWrite;

enum Kind {
    Plain(TcpStream),
    //    SecureClient(Box<SecureStream<ClientSession>>),
    //    SecureServer(Box<SecureStream<ServerSession>>),
}

pub fn plain(tcp: TcpStream) -> Socket {
    Socket {
        local_addr: tcp.local_addr().expect("tcp stream has no local address"),
        peer_addr: tcp.peer_addr().expect("tcp stream has no peer address"),
        kind: Kind::Plain(tcp),
    }
}

pub struct Socket {
    local_addr: SocketAddr,
    peer_addr: SocketAddr,
    kind: Kind,
}

impl Socket {
    pub fn tcp_shutdown(&mut self, how: Shutdown) -> io::Result<()> {
        trace!("{:?}.tcp_shutdown({:?})", self, how);
        match self.kind {
            Kind::Plain(ref mut stream) => TcpStream::shutdown(stream, how),
            //            Kind::SecureClient(ref mut stream) => stream.tcp_shutdown(how),
            //            Kind::SecureServer(ref mut stream) => stream.tcp_shutdown(how),
        }
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }
}

impl fmt::Debug for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::Plain(_) => {
                f.debug_struct("Plain")
                    .field("peer", &self.peer_addr)
                    .field("local", &self.local_addr)
                    .finish()
            }
            //            Kind::SecureClient(_) => {
            //                f.debug_struct("SecureClient")
            //                    .field("peer", &self.peer_addr)
            //                    .field("local", &self.local_addr)
            //                    .finish()
            //            }
            //            Kind::SecureServer(_) => {
            //                f.debug_struct("SecureServer")
            //                    .field("peer", &self.peer_addr)
            //                    .field("local", &self.local_addr)
            //                    .finish()
            //            }
        }
    }
}

/// Reads the socket without blocking.
impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        trace!("{:?}.read({})", self, buf.len());
        match self.kind {
            Kind::Plain(ref mut stream) => stream.read(buf),
            //            Kind::SecureClient(ref mut stream) => stream.read(buf),
            //            Kind::SecureServer(ref mut stream) => stream.read(buf),
        }
    }
}

/// Writes to the socket without blocking.
impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        trace!("{:?}.write({})", self, buf.len());
        match self.kind {
            Kind::Plain(ref mut stream) => stream.write(buf),
            //            Kind::SecureClient(ref mut stream) => stream.write(buf),
            //            Kind::SecureServer(ref mut stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        trace!("{:?}.flush()", self);
        match self.kind {
            Kind::Plain(ref mut stream) => stream.flush(),
            //            Kind::SecureClient(ref mut stream) => stream.flush(),
            //            Kind::SecureServer(ref mut stream) => stream.flush(),
        }
    }
}

/// Closes the write-side of a stream.
impl AsyncWrite for Socket {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        trace!("{:?}.shutdown()", self);
        match self.kind {
            Kind::Plain(ref mut stream) => AsyncWrite::shutdown(stream),
            //            Kind::SecureClient(ref mut stream) => stream.shutdown(),
            //            Kind::SecureServer(ref mut stream) => stream.shutdown(),
        }
    }
}