use crate::api_context::ApiContext;
use crate::ApiClient;
use anyhow::Result;
use serde_json::Value;

pub struct PosGetCustomerApi {
    pub client: &'static ApiClient,
    pub ctx: &'static ApiContext,
}

impl PosGetCustomerApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
            ctx: ApiContext::global(),
        }
    }

    pub async fn pos_get_customer(&self, uuid: &str) -> Result<()> {
        let response: Value = self
            .client
            .get(format!("api/v1/admin/pos/get-customer/{uuid}?customer_type=registered").as_str())
            .send_json()
            .await?;
        self.ctx
            .store_raw("pos_get_customer", "data", response.clone())
            .unwrap();
        Ok(())
    }
}
