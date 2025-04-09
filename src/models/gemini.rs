use crate::types::{AiRequest, Content, Part};
use reqwest::Client;
use std::env;

pub async fn call(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let key = env::var("GEMINI_API_KEY")?;
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        key
    );

    let request = AiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: input.to_string(),
            }],
        }],
    };

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let text = response["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(text)
}