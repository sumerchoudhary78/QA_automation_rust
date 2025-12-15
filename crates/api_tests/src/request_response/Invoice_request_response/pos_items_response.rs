use crate::common::ApiResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PosItemsResponse = ApiResponse<PosItem>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PosItem {
    pub id: i64,
    pub uuid: String,
    #[serde(default)]
    pub subcategory_name: String,
    #[serde(default)]
    pub item_name: String,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub max_discount: f64,
    #[serde(default)]
    pub tax_percentage: f64,
    #[serde(default)]
    pub is_editable: i64,
    #[serde(default)]
    pub pos_item_type: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
