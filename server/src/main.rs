use claude_client::claude::ClaudeClient;
use dotenv::dotenv;
use crate::app::{AppConfig, TakehomeApp};

pub mod app;
pub mod web;
pub mod client;
pub mod finance;

#[tokio::main]
pub async fn main() {
    dotenv().ok().expect("Couldn't parse .env");
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env();
    let claude = ClaudeClient::new()
        .expect("Failed to setup Claude client");

    let app = TakehomeApp::new(config, claude);

    web::serve(app).await
}