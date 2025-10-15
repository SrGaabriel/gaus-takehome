use claude_client::claude::ClaudeClient;
use serde::de::DeserializeOwned;

pub mod stock;

pub async fn send_json_prompt<T : DeserializeOwned>(
    claude: &ClaudeClient,
    model: Option<&str>,
    system_msg: Option<&str>,
    message: &str
) -> Result<T, Box<dyn std::error::Error>> {
    let response = claude.send_message(
        model,
        system_msg.unwrap_or(""),
        message
    ).await?;
    println!("Claude response: {}", response);

    let formatted_response_text = response
        .replace("```json\n", "")
        .replace("\n```", "")
        .replace("```", ""); // fallback in case claude didn't use newlines

    println!("Claude response: {}", formatted_response_text);

    let response: T = serde_json::from_str(&formatted_response_text)
        .expect("Failed to parse Claude response");
    Ok(response)
}