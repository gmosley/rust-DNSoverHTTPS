use dns_parser;

#[derive(Debug)]
/// A rust-dns-https error.
pub enum Error {
    DNSParserErr(dns_parser::Error),
    PacketBuildErr(Vec<u8>),
    InvalidQuestionPacketErr,
    UnsupportedResponseType(u16),
}