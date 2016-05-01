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
use error::Error;

mod structs;
use structs::APIResponse;

use std::io::Read;

use std::net::UdpSocket;
use dns_parser::{Packet, Question, QueryType, Builder, Type, QueryClass, Class, ResponseCode};

use hyper::{Url, Client};
use hyper::header::{Connection, Host};

use std::thread;

/// The IP address of dns.google.com
const GOOGLE_IP: &'static str = "https://4.31.115.251/";

/// The IP and Port to run the server on.
const DNS_SERVER: &'static str = "0.0.0.0:53";


/// Bind a UdpSocket for DNS_SERVER.
/// Listens for DNS packets and returns a response if no errors occur
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
                    match build_response(packet) {
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

/// Builds a response given a packet, and returns the bytes.
/// Note: Only handles A questions. Need to create better errors.
fn build_response(packet: Packet) -> Result<Vec<u8>, Error> {
    if packet.header.questions == 1 {
        let question = &packet.questions[0];
        let api_request = try!(translate_question(&question));
        let api_response = make_request(api_request).unwrap();
        let mut dns_response = Builder::new_response(
            packet.header.id,
            ResponseCode::NoError,
            api_response.TC,
            api_response.RD,
            api_response.RA
        );
        
        // parse questions
        for api_question in &api_response.questions {
            let query_type = QueryType::parse(api_question.question_type).unwrap();
            dns_response.add_question(
                &remove_fqdn_dot(&api_question.name),
                query_type,
                QueryClass::IN
            );
        }

        // parse answers
        if let Some(answers) = api_response.answers {
            for api_answer in answers {
                let data = try!(api_answer.write());
                dns_response.add_answer(
                    &remove_fqdn_dot(&api_answer.name),
                    Type::parse(api_answer.answer_type).unwrap(),
                    Class::IN,
                    api_answer.TTL,
                    data
                );
            }
        }

        let result = dns_response.build();
        match result {
            Ok(bytes) => { 
                // test that the response packet is valid by parsing it
                Packet::parse(&bytes).unwrap();
                Ok(bytes)
            },
            Err(e) => Err(Error::PacketBuildErr(e)),
        }
    } else {
        Err(Error::InvalidQuestionPacketErr)
    }
}

/// Translates a DNS question into a Google API Request
/// This should return a result instead of option
fn translate_question(question: &Question) -> Result<Url, Error> {

    let name = match question.qtype {
        QueryType::A => "A",
        QueryType::AAAA => "AAAA",
        QueryType::PTR => "PTR",
        _ => return Err(Error::UnsupportedResponseType(question.qtype as u16))
    };

    let url_string = GOOGLE_IP.to_owned() + "resolve?type=" +
     name + "&name=" + &question.qname.to_string();

    Ok(Url::parse(&url_string).unwrap())
}

/// Sends an API request to GOOGLE_IP and parses the
/// result into an APIResponse to return.
fn make_request(request: Url) -> Result<APIResponse, Error> {

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

    let api_response : Result<APIResponse, serde_json::Error> = serde_json::from_str(&body);
    api_response.map_err(|e| { Error::SerdeErr(e) })
}

/// Workaround for dns_pasrser, this is done since
/// dns-parser improperly formats fqdns.
fn remove_fqdn_dot (domain_name: &str) -> String {
    let mut domain_name_string = domain_name.to_owned();
    domain_name_string.pop();
    domain_name_string
}
