extern crate tokio;

use std::io::sink;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::net::SocketAddr;

fn main() -> Result<(), Box<std::error::Error>> {
    let addr = "0.0.0.0:9".parse::<SocketAddr>()?;

    let socket = TcpListener::bind(&addr)?;
    eprintln!("Listening on: {}", addr);

    let server = socket
        .incoming()
        .map_err(|e| eprintln!("failed to accept socket; error = {:?}", e))
        .for_each(|socket| {
            let peer_addr = socket.peer_addr().unwrap();
            eprintln!("New peer connection: {}", peer_addr);
            let (reader, _) = socket.split();
            let discard = io::copy(reader, sink()).then(move |result| {
                match result {
                    Ok((n, _, _)) => eprintln!("Discarded {} bytes from peer {}", n, peer_addr),
                    Err(e) => eprintln!("error: {}", e),
                }

                eprintln!("Peer disconnected: {}", peer_addr);

                Ok(())
            });

            tokio::spawn(discard);
            Ok(())
        });

    tokio::run(server);

    Ok(())
}
