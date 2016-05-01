//! Documentation for rust-DNSoverHTTPS.
//! We also have documentation of our fork of
//! https://github.com/david-cao/dns-parser
//! at http://david-cao.github.io/rustdocs/dns_parser/index.html.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate dns_parser;
extern crate hyper;
extern crate serde;
extern crate serde_json;

mod error;
mod structs;
mod worker;

use dns_parser::{Packet};

use std::net::UdpSocket;
use std::thread;

/// The IP and Port to run the server on.
const DNS_SERVER: &'static str = "0.0.0.0:53";

/// Bind a UdpSocket for DNS_SERVER.
/// Listens for DNS packets and sends a response if no errors occur
fn main() {

    let bind_attempt = UdpSocket::bind(DNS_SERVER);
    if bind_attempt.is_err() {
        panic!("Unable to bind {:?}. Make sure you have sufficient user permissions", DNS_SERVER);
    }
    let socket = bind_attempt.unwrap();

    println!("{} bound", DNS_SERVER);

    let mut buf = [0; 512];

    loop {
        let socket_result = socket.recv_from(&mut buf);

        match socket_result {
            Ok((amt, src)) => {
                let socket = socket.try_clone().unwrap();
                thread::spawn(move || {
                    let buf = &mut buf[..amt];
                    let packet = Packet::parse(&buf).unwrap();
                    let lookup_name = &packet.questions[0].qname.to_string();
                    match worker::build_response(packet) {
                        Ok(response_packet) => {
                            socket.send_to(&response_packet, &src).unwrap();
                            println!("OK: {:?} {}", &lookup_name, &src);
                        },
                        Err(e) => {
                            println!("ERROR: {:?} {:?} {}", &lookup_name, e, &src);
                        }
                    }
                });
            }
            Err(e) => panic!("Error receiving datagram: {}", e),
        }
    }
}