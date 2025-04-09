mod gemini;

pub async fn call_ai(model: &str, input: &str) -> Result<String, Box<dyn std::error::Error>> {
    match model {
        "gemini" => gemini::call(input).await,
        _ => Err("Unknown model".into()),
    }
}