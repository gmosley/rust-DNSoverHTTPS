extern crate dns_parser;
extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::json::Json;

use std::io::Read;

use std::net::UdpSocket;
use dns_parser::{Packet, Name, Question, QueryType};

use hyper::Client;
use hyper::header::{Connection, Host};

// TODO: figure out best way to get initial dns.google.com IP
const GOOGLE_IP: &'static str = "https://4.31.115.237/";

const DNS_SERVER: &'static str = "127.0.0.1:53";


fn main() {

    let socket = UdpSocket::bind(DNS_SERVER).unwrap();

    println!("{} bound", DNS_SERVER);

    let mut buf = [0; 512];

    loop {
        let socket_result = socket.recv_from(&mut buf);
        
        match socket_result {
            Ok((amt, src)) => {
                let buf = &mut buf[..amt];
                let packet = Packet::parse(&buf).unwrap();
                let id = packet.header.id;
                for question in packet.questions {
                    make_request(&question, id);
                }
            }
            Err(e) => panic!("Error receiving datagram: {}", e),
        }
    }
}

fn make_request(question : &Question, id: u16) {

    // TODO: support all QueryTypes
    if question.qtype != QueryType::A {
        return;
    }

    let mut client = Client::new();

    let url = GOOGLE_IP.to_owned() + "resolve?name=" + &question.qname.to_string();
    println!("\n{}\n", &url);

    let mut res = client.get(&url)
        .header(Host{
            hostname: "dns.google.com".to_owned(),
            port: None,
        })
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("{}", &body);

}
