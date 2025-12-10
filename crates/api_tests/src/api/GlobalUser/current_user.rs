use crate::api_context::ApiContext;
use crate::client::ApiClient;
use anyhow::Result;
use serde_json::Value;

pub struct CurrentUserApi {
    client: &'static ApiClient,
    ctx: &'static ApiContext,
}

impl CurrentUserApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
            ctx: ApiContext::global(),
        }
    }

    pub async fn get_current_user(&self, uuid: &str) -> Result<Value> {
        let response: Value = self
            .client
            .get(
                format!(
                    "api/v1/admin/elder/{uuid}?includes=user_relationships,consumer_addresses",
                    uuid = uuid
                )
                .as_str(),
            )
            .send_json()
            .await?;
        self.ctx
            .store_raw("current_user", "data", response.clone())
            .unwrap();
        Ok(response)
    }
}
