use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub base_url: String,
    pub api_base_url: String,
    pub email: String,
    pub password: String,
    pub mobile_number: String,
    pub otp: String,
    pub elder_number: String,
    pub country_code: String,
}

static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    dotenv().ok();
    let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let config_path = format!("config/{}.yaml", env);
    let content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read config file: {}", config_path));

    let mut config: AppConfig = serde_yaml::from_str(&content).expect("Invalid config format");

    config.email = env::var("WMS_EMAIL").expect("WMS_EMAIL not set");
    config.password = env::var("WMS_PASSWORD").expect("WMS_PASSWORD not set");
    config.mobile_number = env::var("WMS_PHONE").expect("WMS_PHONE not set");
    config.otp = env::var("WMS_OTP").expect("WMS_OTP not set");
    config.elder_number = env::var("ELDER_NUMBER").expect("ELDER_NUMBER not set");

    config
});

pub fn get_config() -> &'static AppConfig {
    &CONFIG
}
