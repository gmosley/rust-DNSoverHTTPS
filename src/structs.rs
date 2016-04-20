#![allow(non_snake_case)]

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
    pub answers: Vec<APIAnswer>,
}