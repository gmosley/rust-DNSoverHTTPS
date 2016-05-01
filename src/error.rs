use dns_parser;
use hyper;
use serde_json;

#[derive(Debug)]
/// A rust-dns-https error.
pub enum Error {
    DNSParserErr(dns_parser::Error),
    HyperErr(hyper::Error),
    PacketBuildErr(Vec<u8>),
    SerdeErr(serde_json::Error),
    InvalidQuestionPacketErr,
    UnsupportedResponseType(u16),
}