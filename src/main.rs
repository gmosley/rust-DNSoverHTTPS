extern crate dns_parser;
extern crate hyper;

use std::io::Read;

use std::net::UdpSocket;
use dns_parser::{Packet, Name};

use hyper::Client;
use hyper::header::{Connection, Host};

fn main() {

    make_request();

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
                    let question = &packet.questions[0];
                    println!("{} {:?}\n", &question.qname.to_string(), question.qtype);
                }
            }
            Err(e) => panic!("Error receiving datagram: {}", e),
        }
    }
}

fn make_request() {
    let mut client = Client::new();

    let mut res = client.get("https://4.31.115.237/resolve?name=seas.upenn.edu.")
        .header(Host{
            hostname: "dns.google.com".to_owned(),
            port: None,
        })
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
