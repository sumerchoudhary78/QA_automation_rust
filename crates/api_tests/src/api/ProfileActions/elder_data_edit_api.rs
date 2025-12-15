use crate::api_context::ApiContext;
use crate::client::ApiClient;
use crate::factories;
use crate::request_response::elder_data_edit::ElderDataEditRequest;
use crate::request_response::elder_data_edit::ElderDataEditResponse;

use anyhow::Result;
use serde_json::Value;

pub struct ElderDataEditApi {
    client: &'static ApiClient,
    ctx: &'static ApiContext,
}

impl ElderDataEditApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
            ctx: ApiContext::global(),
        }
    }

    pub async fn update_elder_data(
        &self,
        request: ElderDataEditRequest,
    ) -> Result<ElderDataEditResponse> {
        let response = self
            .client
            .put(
                format!(
                    "api/v1/admin/sales/elder/lead-profile/detail-of-person-calling/{currentElderIdentifier}",
                    currentElderIdentifier = request.currentElderIdentifier
                )
                .as_str(),
            )
            .json(&request)
            .send_json::<ElderDataEditResponse>()
            .await;

        response
    }

    pub async fn update_elder_data_with_factory(&self) -> Result<ElderDataEditResponse> {
        let response: Value = self.ctx.get_raw("current_user", "data")?;
        let user = response["data"]
            .as_array()
            .and_then(|arr| arr.first())
            .ok_or_else(|| anyhow::anyhow!("'data' array is empty or not found"))?;

        let uuid = user["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("'id' field not found or not a string"))?;

        let consumeraddress_uuid = user["consumer_addresses"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|addr| addr["id"].as_str())
            .or_else(|| {
                user["owner"]["consumer_addresses"]
                    .as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|addr| addr["id"].as_str())
            })
            .unwrap_or("");

        let mobile_number = user["mobile_number"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("'mobile_number' not found or not a string"))?;

        self.update_elder_data(ElderDataEditRequest {
            currentElderIdentifier: uuid.to_string(),
            consumeraddress_uuid: consumeraddress_uuid.to_string(),
            phone_no: mobile_number.to_string(),
            is_nok_elder_calling: "elder".to_string(),
            email: factories::TestDataFactory::email(),
            reasons_for_downloading_app: None,
            first_name: "Test".to_string(),
            age: Some(58),
            languages: None,
            last_name: factories::TestDataFactory::last_name(),
            gender: 1,
            current_medical_conditions: None,
            pincode: None,
            address_line_1: None,
            address_line_2: None,
            city: None,
            state: None,
            locality: None,
            country: None,
            new_selected_ep_uuid: None,
            r#override: None,
        })
        .await
    }
}
