use crate::api_context::ApiContext;
use crate::ApiClient;
use anyhow::Result;
use serde_json::json;
pub struct VoidInvoiceApi {
    client: &'static ApiClient,
    ctx: &'static ApiContext,
}

impl VoidInvoiceApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
            ctx: ApiContext::global(),
        }
    }

    pub async fn invoice_list(&self, uuid: &str) -> Result<()> {
        let response = self
            .client
            .get(format!("/api/v1/admin/invoice-listing-wms?uuid={uuid}&page=1&limit=20").as_str())
            .send_json()
            .await;
        self.ctx.store_raw(
            "invoice_list",
            "data",
            response.as_ref().ok().cloned().unwrap(),
        )?;
        Ok(())
    }

    pub async fn void_invoice(&self, invoice_uuid: &str) -> Result<String> {
        let response = self
            .client
            .put("/api/v1/admin/void-receipt")
            .json(json!({"invoice_uuid": invoice_uuid}).as_object().unwrap())
            .send_json::<String>()
            .await;

        response
    }

    pub async fn void_invoice_with_factory(&self) -> Result<()> {
        let response = self
            .ctx
            .get_raw("current_user", "data")
            .map_err(|e| anyhow::anyhow!("Failed to get current_user: {}", e))?;

        let user = response["data"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("current_user data is not an array"))?
            .first()
            .ok_or_else(|| anyhow::anyhow!("No user found in current_user data"))?;

        let uuid = user["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("User id is not a string"))?;

        self.invoice_list(uuid).await?;

        println!(
            "[DEBUG] invoice_list: {:?}",
            self.ctx.get_raw("invoice_list", "data").unwrap()
        );

        let invoice_data = self
            .ctx
            .get_raw("invoice_list", "data")
            .map_err(|e| anyhow::anyhow!("Failed to get invoice_list: {}", e))?;

        let invoices = invoice_data["data"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("invoice_list data is not an array"))?;

        if invoices.is_empty() {
            return Err(anyhow::anyhow!("No invoices found to void"));
        }

        let invoice_uuid = invoices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No invoice in list"))?;

        let uuid = invoice_uuid["receipt_uuid"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("receipt_uuid is not a string"))?;

        self.void_invoice(uuid).await?;
        Ok(())
    }
}
