use crate::api::Invoices::pos_checkout::PosCheckoutApi;
use crate::api_context::ApiContext;
use crate::request_response::Invoice_request_response::pos_checkout_request::{
    Payment, PosCheckoutRequest,
};
use crate::ApiClient;
use anyhow::Result;

pub async fn pos_checkout_test() -> Result<()> {
    let ctx = ApiContext::global();
    let client = ApiClient::global();
    let pos_checkout_api = PosCheckoutApi::new();
    let pos_checkout_request = PosCheckoutRequest {
        pos_node_id: "".to_string(),
        cart_id: "".to_string(),
        posReceipt_id: "".to_string(),
        cart_items: vec![],
        cart_type: "".to_string(),
        concierge_ticket: vec![],
        payment: Payment {
            amount: 0.0,
            method: "".to_string(),
            send_link_to_email: "".to_string(),
            send_link_to_mobile: "".to_string(),
        },
        share_invoice: vec![],
        start_date: "".to_string(),
        end_date: "".to_string(),
        invoice_date: "".to_string(),
        invoice_payment_due_date: "".to_string(),
        place_of_supply: "".to_string(),
        total_discount: 0.0,
        total_mrp: 0.0,
        total_gst: 0.0,
        bill_total: 0.0,
        source: "".to_string(),
        lead_uuid: None,
    };
    pos_checkout_api.pos_checkout(pos_checkout_request).await?;
    Ok(())
}
