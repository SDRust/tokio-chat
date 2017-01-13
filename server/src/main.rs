// A tiny async echo server with tokio-core

extern crate futures;
extern crate tokio_core;

use futures::{Future, Stream};
use tokio_core::io::{copy, Io};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

fn main() {
    // Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Bind the server's socket
    let addr = "0.0.0.0:1337".parse().unwrap();
    let sock = TcpListener::bind(&addr, &handle).unwrap();

    // Pull out a stream of sockets for incoming connections
    let server = sock.incoming().for_each(|(sock, _)| {
        // Split up the reading and writing parts of the
        // socket
        let (reader, writer) = sock.split();

        let mut buf = vec![0; 1024 * 8];
        reader.try_read(&buf).unwrap();

        // Spawn the future as a concurrent task
//        handle.spawn(handle_conn);

        Ok(())

    });

    // Spin up the server on the event loop
    core.run(server).unwrap();
}