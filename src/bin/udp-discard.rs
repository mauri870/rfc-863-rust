#[macro_use]
extern crate futures;
extern crate tokio;

use std::net::SocketAddr;
use std::io;

use tokio::prelude::*;
use tokio::net::UdpSocket;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            let (n, addr) = try_ready!(self.socket.poll_recv_from(&mut self.buf));
            eprintln!("Discarded {} bytes from peer {}", n, addr);
        }
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let addr = "127.0.0.1:9".parse::<SocketAddr>()?;

    let socket = UdpSocket::bind(&addr)?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket: socket,
        buf: vec![0; 1024],
    };

    // This starts the server task
    tokio::run(server.map_err(|e| eprintln!("server error = {:?}", e)));
    Ok(())
}
