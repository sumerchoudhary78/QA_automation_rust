use serde::Deserialize;
use anyhow::Result;
use std::fs;
use once_cell::sync::Lazy;
use dotenvy::dotenv;
use std::env;

/// Struct for environment-specific configurations
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub base_url: String,
    pub browser: Option<String>,
    pub api_base_url: Option<String>,
}

static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    dotenv().ok(); // Load .env if present
    let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let config_path = format!("config/{}.yaml", env);
    let content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read config file: {}", config_path));
    serde_yaml::from_str(&content).expect("Invalid config format")
});

pub fn get_config() -> &'static AppConfig {
    &CONFIG
}
