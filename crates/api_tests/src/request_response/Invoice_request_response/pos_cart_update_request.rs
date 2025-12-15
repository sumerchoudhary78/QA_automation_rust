use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Isize(i64),
    Usize(f64),
    Array(Vec<Value>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePosCartItemRequest {
    pub customer_id: Value,
    pub customer_type: Value,
    pub node_id: Value,
    pub cart_items: Vec<Items>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Items {
    New {
        node_item_id: Value,
        price: Value,
        quantity: Value,
        discount_percentage: Value,
    },
    Update {
        uuid: Value,
        node_item_id: Value,
        price: Value,
        quantity: Value,
        discount_percentage: Value,
    },
}
