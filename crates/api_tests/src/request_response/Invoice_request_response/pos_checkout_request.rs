use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PosCheckoutRequest {
    pub pos_node_id: String,
    pub cart_id: String,
    pub posReceipt_id: String,
    pub customer_invoice_name: String,
    pub cart_items: Vec<Items>,
    pub cart_type: String,
    pub concierge_ticket: Vec<String>,
    pub payment: Payment,
    pub share_invoice: Vec<ShareInvoice>,
    pub start_date: String,
    pub end_date: String,
    pub invoice_date: String,
    pub invoice_payment_due_date: String,
    pub place_of_supply: String,
    pub total_discount: f64,
    pub total_mrp: f64,
    pub total_gst: f64,
    pub bill_total: f64,
    pub source: String,
    pub lead_uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Items {
    pub cart_item_id: String,
    pub discount_percent: i64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payment {
    pub amount: f64,
    pub method: String,
    pub send_link_to_email: String,
    pub send_link_to_mobile: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShareInvoice {
    pub phone_number: String,
    pub email: String,
}
