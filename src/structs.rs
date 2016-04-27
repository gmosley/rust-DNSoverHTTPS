#![allow(non_snake_case)]


use error::Error;

use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct APIQuestion {
    pub name : String,
    #[serde(rename="type")]
    pub question_type : u16,
}

#[derive(Deserialize, Debug)]
pub struct APIAnswer {
    pub name : String,
    #[serde(rename="type")]
    pub answer_type: u16,
    pub TTL: u32,
    pub data : String,
}



impl APIAnswer {
    pub fn write(&self) -> Result<Vec<u8>, Error> {
        match self.answer_type {
            1  =>  {
                let ip = Ipv4Addr::from_str(&self.data).unwrap();
                Ok(ip.octets().to_vec())
            },
            5  => {
                let mut data : Vec<u8> = Vec::new();
                let name = &self.data;
                //println!("CNAME: {:?}", name);
                for label in name.split('.') {
                    let size = label.len() as u8;
                    data.push(size);
                    data.extend(label.as_bytes());
                }
                Ok(data)
            }
            28 => {
                let ip = Ipv6Addr::from_str(&self.data).unwrap();
                let mut ipv6_bytes : Vec<u8> = Vec::new();
                // THIS REALLY NEEDS TO BE IMPROVED
                for segment in ip.segments().iter() {
                    let upper = segment >> 8;
                    let lower = segment & 0b0000_0000_1111_1111;
                    ipv6_bytes.push(upper as u8);
                    ipv6_bytes.push(lower as u8);
                }
                Ok(ipv6_bytes)
            }
            x => Err(Error::UnsupportedResponseType(x)) 
        }
    }
}

/// The Main Google Response Struct. Can contain APIQuestions and APIResponses.
#[derive(Deserialize, Debug)]
pub struct APIResponse {
    pub Status : u32,
    pub TC : bool,
    pub RD : bool, // Should always be true for Google Public DNS
    pub RA : bool, // Should always be true for Google Public DNS
    pub AD : bool,
    pub CD : bool,

    #[serde(rename="Question")]
    pub questions: Vec<APIQuestion>,

    #[serde(rename="Answer")]
    pub answers: Option<Vec<APIAnswer>>,
}