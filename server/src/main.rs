extern crate tokio_chat_common;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate native_tls;
extern crate tokio_tls;

use native_tls::{Pkcs12};

use tokio_core::reactor::Core;
use tokio_core::net::{TcpListener};
use tokio_tls::{TlsConnectorExt, TlsAcceptorExt};

use native_tls::TlsAcceptor;

use std::sync::Arc;
use std::fs::File;
use std::io::{Read};
//use std::net::{TcpStream, TcpListener};

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut file = File::open("../key_and_cert.p12").unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = Pkcs12::from_der(&pkcs12, "hunter2").unwrap();

    let acceptor = TlsAcceptor::builder(pkcs12).unwrap().build().unwrap();
    let acceptor = Arc::new(acceptor);

    let server = TcpListener::bind(&("0.0.0.0:1337".parse().unwrap()), &handle).unwrap();

/*    let listener = TcpListener::bind("0.0.0.0:1337").unwrap();
    let socket = listener.accept();
    acceptor.build(socket);*/

//    let new_conn = acceptor.accept_async(listener);


}
