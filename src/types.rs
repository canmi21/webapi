use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AiRequest {
    pub contents: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Deserialize)]
pub struct AiResponse {
    pub text: String,
}