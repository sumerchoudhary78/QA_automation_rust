use crate::api_context::ApiContext;
use crate::request_response::Invoice_request_response::pos_cart_update_request::{
    Items, UpdatePosCartItemRequest, Value as RequestValue,
};
use crate::ApiClient;
use anyhow::Result;
use serde_json::Value;

pub struct UpdatePosCartItemApi {
    pub client: &'static ApiClient,
    pub ctx: &'static ApiContext,
}

impl UpdatePosCartItemApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
            ctx: ApiContext::global(),
        }
    }

    pub async fn update_pos_cart_item(
        &self,
        update_pos_cart_item_request: UpdatePosCartItemRequest,
    ) -> Result<()> {
        let response: Value = self
            .client
            .post("/api/v1/admin/update-pos-cart-item-wms")
            .json(&update_pos_cart_item_request)
            .send_json()
            .await?;
        self.ctx
            .store_raw("update_pos_cart_item", "data", response.clone())
            .unwrap();
        Ok(())
    }

    pub async fn update_pos_cart_item_with_factory(&self, node_id: &str) -> Result<()> {
        let response = self.ctx.get_raw("current_user", "data").unwrap();

        let user = response["data"].as_array().unwrap().first().unwrap();
        let uuid = user["id"].as_str().unwrap();
        let pos_item: Value = self
            .ctx
            .get_row_map("pos_items_all", "data", 17009)
            .unwrap();
        let node_item_id = pos_item["uuid"].as_str().unwrap();

        self.update_pos_cart_item(UpdatePosCartItemRequest {
            customer_id: RequestValue::String(uuid.to_string()),
            customer_type: RequestValue::String("registered".to_string()),
            node_id: RequestValue::String(node_id.to_string()),
            cart_items: vec![Items::New {
                node_item_id: RequestValue::String(node_item_id.to_string()),
                price: RequestValue::Usize(0.00),
                quantity: RequestValue::Usize(1.00),
                discount_percentage: RequestValue::Usize(0.00),
            }],
        })
        .await?;
        Ok(())
    }

    pub async fn update_pos_cart_item_with_factory_and_price(
        &self,
        node_id: &str,
        price: &f64,
    ) -> Result<()> {
        let response = self.ctx.get_raw("current_user", "data").unwrap();

        let user = response["data"].as_array().unwrap().first().unwrap();
        let uuid = user["id"].as_str().unwrap();
        let pos_item: Value = self
            .ctx
            .get_row_map("pos_items_all", "data", 17009)
            .unwrap();
        let node_item_id = pos_item["uuid"].as_str().unwrap();

        let update_pos_cart_response = self.ctx.get_raw("update_pos_cart_item", "data").unwrap();
        println!(
            "[DEBUG] update_pos_cart_response: {:?}",
            update_pos_cart_response
        );

        let snapshot_id = update_pos_cart_response["data"]["data"]
            .as_array()
            .expect("Expected data.data to be an array")
            .first()
            .expect("Expected at least one item in data.data array")["uuid"]
            .as_str()
            .expect("Expected uuid to be a string");

        self.update_pos_cart_item(UpdatePosCartItemRequest {
            customer_id: RequestValue::String(uuid.to_string()),
            customer_type: RequestValue::String("registered".to_string()),
            node_id: RequestValue::String(node_id.to_string()),
            cart_items: vec![Items::Update {
                uuid: RequestValue::String(snapshot_id.to_string()),
                node_item_id: RequestValue::String(node_item_id.to_string()),
                price: RequestValue::Usize(*price),
                quantity: RequestValue::Isize(1),
                discount_percentage: RequestValue::Usize(0.00),
            }],
        })
        .await?;
        Ok(())
    }
}
