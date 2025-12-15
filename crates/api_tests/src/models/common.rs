use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub data: Vec<T>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
}
