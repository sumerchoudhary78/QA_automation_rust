use crate::{api_context::ApiContext, ApiClient};
use anyhow::Result;
use serde_json::Value;

pub struct PosItemsAllApi {
    client: &'static ApiClient,
    ctx: &'static ApiContext,
}

impl PosItemsAllApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
            ctx: ApiContext::global(),
        }
    }

    pub async fn get_pos_items_all(&self, pos_node_id: &str) -> Result<Value> {
        let response: Value = self
            .client
            .get(
                format!(
                    "api/v1/admin/pos-items-all/{pos_node_id}",
                    pos_node_id = pos_node_id
                )
                .as_str(),
            )
            .send_json()
            .await?;
        self.ctx
            .store_raw("pos_items_all", "data", response.clone())
            .unwrap();
        Ok(response)
    }
}
