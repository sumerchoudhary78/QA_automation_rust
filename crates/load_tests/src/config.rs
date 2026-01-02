use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadConfig {
    pub base_url: String,
    pub users: u32,
    pub run_time: String,
    pub hatch_rate: u32,
    pub request_timeout: u64,
    pub think_time: (u64, u64),
}

impl Default for LoadConfig {
    fn default() -> Self {
        let base_url = lib_test_helpers::config::get_config().api_base_url.clone();

        Self {
            base_url,
            users: 10,
            run_time: "1m".into(),
            hatch_rate: 2,
            request_timeout: 30,
            think_time: (500, 2000),
        }
    }
}

impl LoadConfig {
    pub fn new(base_url: String, users: u32, run_time: String) -> Self {
        Self {
            base_url,
            users,
            run_time,
            ..Default::default()
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: LoadConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioWeights {
    pub auth_flow: usize,
    pub invoice_flow: usize,
    pub browse_flow: usize,
}

impl Default for ScenarioWeights {
    fn default() -> Self {
        Self {
            auth_flow: 1,
            invoice_flow: 2,
            browse_flow: 3,
        }
    }
}
