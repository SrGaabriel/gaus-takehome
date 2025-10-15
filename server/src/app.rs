use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use claude_client::claude::ClaudeClient;
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

#[derive(Clone)]
pub struct TakehomeApp {
    // todo: make this Arc if it ever gets too big
    pub http_client: Arc<ClientWithMiddleware>,
    pub config: AppConfig,
    pub claude: Arc<ClaudeClient>,
}

impl TakehomeApp {
    pub fn new(config: AppConfig, claude: ClaudeClient) -> Self {
        Self {
            http_client: Arc::new(ClientBuilder::new(Client::new()).build()),
            config,
            claude: Arc::new(claude),
        }
    }
}

#[derive(Clone)]
pub struct AppConfig {
    pub rest_addr: String,
    pub alpha_vantage_api_key: String,
    pub claude_model: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            rest_addr: get_required_env("REST_ADDR"),
            alpha_vantage_api_key: get_required_env("ALPHA_VANTAGE_API_KEY"),
            claude_model: get_required_env("ANTHROPIC_MODEL"),
        }
    }
}

fn get_required_env<T : FromStr>(var: &str) -> T where <T as FromStr>::Err: Debug {
    std::env::var(var)
        .expect(&format!("Missing required environment variable: {}", var))
        .parse()
        .expect(&format!("Environment variable formated incorrectly: {}", var))
}