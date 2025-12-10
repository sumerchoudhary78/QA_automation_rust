use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub emi_status: i32,
    pub pos_node_id: String,
    pub cart_id: String,
    pub pos_receipt_id: String,
    pub customer_invoice_name: String,
    pub cart_items: Vec<CartItem>,
    pub cart_type: String,
    pub concierge_ticket: Vec<String>,
    pub payment: PaymentInfo,
    pub share_invoice: Vec<ShareInfo>,
    pub start_date: String,
    pub end_date: String,
    pub invoice_date: String,
    pub invoice_payment_due_date: String,
    pub place_of_supply: String,
    pub total_discount: i32,
    pub total_mrp: f64,
    pub total_gst: f64,
    pub bill_total: f64,
    pub source: String,
    pub lead_uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub cart_item_id: String,
    pub discount_percent: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInfo {
    pub amount: i32,
    pub method: String,
    pub send_link_to_email: String,
    pub send_link_to_mobile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareInfo {
    pub phone_number: String,
    pub email: String,
}
