use std::io::{self, Write};
use webapi::call_ai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        let response = call_ai("gemini", input).await?;
        println!("{}", response);
    }

    Ok(())
}