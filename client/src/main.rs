extern crate futures;
extern crate tokio_core;
extern crate tokio_chat_common;
extern crate serde_json;
use futures::Future;
use std::io;
use std::net::ToSocketAddrs;
use tokio_core::net::{TcpStream,TcpStreamNew};
use tokio_core::reactor::Core;
use tokio_chat_common::*;

fn main() {
    let address = "127.0.0.1:1337";
    let nickname = "team1";

    let message = Message {
        message: "Hello, world!".to_string(),
        from:    "team1".to_string(),
        time:    "2017-01-12T20:00:00-0800".to_string()
    };

    let addr = address.to_socket_addrs().unwrap().next().unwrap();

    // connect to server
    let mut core = Core::new().unwrap();
    let socket = TcpStream::connect(&addr, &core.handle());

    let handle = login(&core, &socket);
    // send login Command
/*
    let response = login_request.and_then(|(socket, _)| {
        tokio_core::io::read_to_end(socket, Vec::new())
    });
*/


    // check login command response

    // send a message

    // display received messages

    //println!("{}", serde_json::to_string(&message).unwrap());
}

fn login (core: &Core, socket: &TcpStreamNew) -> CommandResponse {

    let login_cmd = Command {
        command:  "login".to_string(),
        nickname: "team1".to_string()
    };

    let login_message = serde_json::to_string(&login_cmd).unwrap();

    let login_request = socket.and_then(move |socket| {
         tokio_core::io::write_all(socket, login_message.as_bytes())
     });

    let handle = login_request.and_then(move |(socket, _)| {
        tokio_core::io::read_to_end(socket, Vec::new());//, b'\n', Vec::new())
    } )
    .and_then(
        |(socket,  response)| {
            serde_json::from_str::<CommandResponse>(&response)
    } );

    let (_, data) = core.run(handle).unwrap();
    data
}
