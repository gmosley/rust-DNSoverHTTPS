#[derive(Deserialize, Debug)]
pub struct APIQuestion {
    name : String,
    #[serde(rename="type")]
    question_type : u32,
}

#[derive(Deserialize, Debug)]
pub struct APIAnswer {
    name : String,
    #[serde(rename="type")]
    answer_type: u32,
    TTL: u32,
    data : String,
}

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    Status : u32,
    TC : bool,
    RD : bool, // Should always be true for Google Public DNS
    RA : bool, // Should always be true for Google Public DNS
    AD : bool,
    CD : bool,

    #[serde(rename="Question")]
    questions: Vec<APIQuestion>,

    #[serde(rename="Answer")]
    answers: Vec<APIAnswer>,
}