extern crate dns_parser;

use std::net::UdpSocket;
use dns_parser::{Packet, Name};

fn main() {

    let socket = UdpSocket::bind("127.0.0.1:53").unwrap();

    println!("127.0.0.1:53 bound");

    let mut buf = [0; 512];

    loop {
        let socket_result = socket.recv_from(&mut buf);

        match socket_result {
            Ok((amt, src)) => {
                let buf = &mut buf[..amt];
                let packet = Packet::parse(&buf).unwrap();
                if packet.questions.len() == 1 {
                    println!("{}\n", &packet.questions[0].qname.to_string());
                }
            }
            Err(e) => panic!("Error receiving datagram: {}", e),
        }
    }
}
