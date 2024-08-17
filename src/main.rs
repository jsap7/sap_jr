use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};
use std::process::Command;

#[derive(Serialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the API key from the environment variable
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found in environment");

    let client = Client::new();
    let mut conversation = vec![Message {
        role: "system".into(),
        content: "You are a helpful assistant.".into(),
    }];

    println!("Start chatting with the assistant (type 'exit' to stop):");

    loop {
        let mut user_input = String::new();
        print!("You: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;

        let user_input = user_input.trim().to_string();
        if user_input.to_lowercase() == "exit" {
            break;
        }

        conversation.push(Message {
            role: "user".into(),
            content: user_input.clone(),
        });

        let request_body = ChatRequest {
            model: "gpt-3.5-turbo".into(), // or "gpt-4" depending on your subscription
            messages: conversation.clone(),
        };

        let response: ChatResponse = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()?
            .json()?;

        let ai_response = &response.choices[0].message.content;
        println!("Assistant: {}", ai_response);

        // Use macOS's built-in 'say' command to speak the response
        Command::new("say")
            .arg(ai_response)
            .output()
            .expect("Failed to execute 'say' command");

        conversation.push(Message {
            role: "assistant".into(),
            content: ai_response.clone(),
        });
    }

    Ok(())
}
