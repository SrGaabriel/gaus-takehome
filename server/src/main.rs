use claude_client::claude::ClaudeClient;
use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::app::{AppConfig, TakehomeApp};

pub mod app;
pub mod web;
pub mod client;
pub mod finance;

#[tokio::main]
pub async fn main() {
    dotenv().ok().expect("Couldn't parse .env");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=info,diesel=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();
    let claude = ClaudeClient::new()
        .expect("Failed to setup Claude client");

    let app = TakehomeApp::new(config, claude);

    web::serve(app).await
}