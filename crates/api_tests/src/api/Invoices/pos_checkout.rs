use crate::api_context::ApiContext;
use crate::request_response::Invoice_request_response::pos_checkout_request::Items;
use crate::request_response::Invoice_request_response::pos_checkout_request::Payment;
use crate::request_response::Invoice_request_response::pos_checkout_request::PosCheckoutRequest;
use crate::request_response::Invoice_request_response::pos_checkout_request::ShareInvoice;
use crate::ApiClient;
use anyhow::Result;
use serde_json::Value;
pub struct PosCheckoutApi {
    pub client: &'static ApiClient,
    pub ctx: &'static ApiContext,
}

impl PosCheckoutApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
            ctx: ApiContext::global(),
        }
    }

    pub async fn pos_checkout(&self, request: PosCheckoutRequest) -> Result<()> {
        let response: Value = self
            .client
            .post("api/v1/admin/pos-checkout")
            .json(&request)
            .send_json()
            .await?;
        self.ctx
            .store_raw("pos_checkout", "data", response.clone())
            .unwrap();
        Ok(())
    }

    pub async fn pos_checkout_with_factory(&self, node_id: &str) -> Result<()> {
        let cart_data = self.ctx.get_raw("update_pos_cart_item", "data").unwrap();
        let cart_id = cart_data["data"]["cart_uuid"].as_str().unwrap();
        let total_mrp = cart_data["data"]["total_mrp"].as_f64().unwrap();

        let cart_items = cart_data["data"]["data"].as_array().unwrap();
        let cart_item_id = cart_items.first().unwrap()["uuid"].as_str().unwrap();

        self.pos_checkout(PosCheckoutRequest {
            pos_node_id: node_id.to_string(),
            cart_id: cart_id.to_string(),
            posReceipt_id: "".to_string(),
            customer_invoice_name: "".to_string(),
            cart_items: vec![Items {
                cart_item_id: cart_item_id.to_string(),
                discount_percent: 0,
                description: "".to_string(),
            }],
            cart_type: "service".to_string(),
            concierge_ticket: vec![],
            payment: Payment {
                amount: 0.0,
                method: "link".to_string(),
                send_link_to_email: "scjakhar78@gmail.com".to_string(),
                send_link_to_mobile: "8890349935".to_string(),
            },
            share_invoice: vec![ShareInvoice {
                phone_number: "8890349935".to_string(),
                email: "scjakhar78@gmail.com".to_string(),
            }],
            start_date: "2023-01-01".to_string(),
            end_date: "2023-01-01".to_string(),
            invoice_date: "2023-01-01".to_string(),
            invoice_payment_due_date: "2023-01-01".to_string(),
            place_of_supply: "19971716-d695-4952-8f8e-bb446759ecb9".to_string(),
            total_discount: 0.0,
            total_mrp: total_mrp,
            total_gst: 0.0,
            bill_total: 0.0,
            source: "rust_auto".to_string(),
            lead_uuid: None,
        })
        .await?;
        Ok(())
    }
}
