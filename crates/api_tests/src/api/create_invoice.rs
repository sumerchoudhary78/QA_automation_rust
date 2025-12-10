use crate::client::ApiClient;
use crate::request_response::create_invoice_request::{
    CartItem, CreateInvoiceRequest, PaymentInfo,
};

use anyhow::Result;
use serde_json::Value;

pub struct CreateInvoiceApi {
    client: &'static ApiClient,
}

impl CreateInvoiceApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
        }
    }

    pub async fn create_invoice(&self, request: CreateInvoiceRequest) -> Result<Value> {
        let response = self
            .client
            .post("/api/v1/admin/pos-checkout")
            .json(&request)
            .send_json()
            .await;

        response
    }

    pub async fn create_invoice_with_factory(
        &self,
        cart_id: String,
        cart_item_id: String,
    ) -> Result<Value> {
        self.create_invoice(CreateInvoiceRequest {
            emi_status: 0,
            pos_node_id: "ea5639b8-0fe6-4676-b455-65e251cbc492".to_string(),
            cart_id: cart_id,
            pos_receipt_id: "".to_string(),
            customer_invoice_name: "".to_string(),
            cart_items: vec![CartItem {
                cart_item_id: cart_item_id,
                discount_percent: 0,
                description: None,
            }],
            cart_type: "service".to_string(),
            concierge_ticket: vec![],
            payment: PaymentInfo {
                amount: 10,
                method: "link".to_string(),
                send_link_to_email: "scjakhar78@gmail.com".to_string(),
                send_link_to_mobile: "8890349935".to_string(),
            },
            share_invoice: vec![],
            start_date: "2025-12-09 12:51:34".to_string(),
            end_date: "2025-12-09 12:51:34".to_string(),
            invoice_date: "2025-12-09 12:51:34".to_string(),
            invoice_payment_due_date: "2025-12-09 12:51:34".to_string(),
            place_of_supply: "19971716-d695-4952-8f8e-bb446759ecb9".to_string(),
            total_discount: 0,
            total_mrp: 0.0,
            total_gst: 0.0,
            bill_total: 0.0,
            source: "wms_profile".to_string(),
            lead_uuid: None,
        })
        .await
    }
}
