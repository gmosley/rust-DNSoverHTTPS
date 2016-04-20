//! Documentation for rust-DNSoverHTTPS.
//! Most of the work so far has been on our fork of
//! https://github.com/david-cao/dns-parser with docs
//! located at http://david-cao.github.io/rustdocs/dns_parser/index.html.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

extern crate dns_parser;
extern crate hyper;

extern crate byteorder;
use byteorder::{BigEndian, ByteOrder};

use std::io::Read;

use std::net::UdpSocket;
use dns_parser::{Packet, Name, Question, QueryType, Builder, Type, QueryClass, Class, ResponseCode};

use hyper::{Url, Client};
use hyper::header::{Connection, Host};

use std::net::Ipv4Addr;

use std::thread;

mod structs;
use structs::{APIResponse, APIQuestion, APIAnswer};

/// The IP address of dns.google.com
const GOOGLE_IP: &'static str = "https://4.31.115.237/";

/// The IP and Port to run the server on. Usually localhost:53
const DNS_SERVER: &'static str = "127.0.0.1:53";


fn main() {

    let socket = UdpSocket::bind(DNS_SERVER).unwrap();

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
                    if let Ok(response_packet) = build_response(packet) {
                        socket.send_to(&response_packet, &src).unwrap();
                    }
                });
            }
            Err(e) => panic!("Error receiving datagram: {}", e),
        }
    }
}

/// Builds a response given a packet, and returns the bytes
/// Need to create better errors.
fn build_response(packet: Packet) -> Result<Vec<u8>, String> {

    if packet.header.questions == 1 {
        let question = &packet.questions[0];
        if let Some(api_request) = translate_question(&question) {
            let api_response = make_request(api_request);
            let mut dns_response = Builder::new_response(
                packet.header.id,
                ResponseCode::NoError,
                api_response.TC,
                api_response.RD,
                api_response.RA
            );
            for api_question in &api_response.questions {
                let query_type = QueryType::parse(api_question.question_type).unwrap();
                dns_response.add_question(
                    &remove_fqdn_dot(&api_question.name),
                    query_type,
                    QueryClass::IN
                );
            }
            for api_answer in &api_response.answers {
                // only handle A responses
                if api_answer.answer_type == 1 {
                    use std::str::FromStr;
                    let ip = Ipv4Addr::from_str(&api_answer.data).unwrap();
                    dns_response.add_answer(
                        &remove_fqdn_dot(&api_answer.name),
                        Type::A,
                        Class::IN,
                        api_answer.TTL,
                        BigEndian::read_u32(&ip.octets())
                    );
                }
            }
            let result = dns_response.build();
            match result {
                Ok(bytes) => return Ok(bytes),
                _ => return Err(String::from("Failed to build packet")),
            }
        }
    }
    Err(String::from("Packet can only have 1 question at the moment!"))
}

/// Translates a DNS question into a Google API Request
fn translate_question(question: &Question) -> Option<Url> {

    // TODO: support all QueryTypes
    if question.qtype != QueryType::A {
        return None;
    }

    let url_string = GOOGLE_IP.to_owned() + "resolve?name=" + &question.qname.to_string();


    match Url::parse(&url_string) {
        Ok(url) => Some(url),
        _ => None
    }
}

fn make_request(request: Url) -> APIResponse {

    let client = Client::new();

    let mut res = client.get(request)
        .header(Host{
            hostname: "dns.google.com".to_owned(),
            port: None,
        })
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let api_response : APIResponse = serde_json::from_str(&body).unwrap();
    api_response

}


/// Workaround for dns_pasrser
fn remove_fqdn_dot (domain_name: &str) -> String {
    let mut domain_name_string = domain_name.to_owned();
    domain_name_string.pop();
    domain_name_string
}
