use crate::client::ApiClient;
use crate::request_response::user_actions::lead::LeadCreateRequest;
use crate::request_response::user_actions::lead::LeadCreateResponse;
use crate::utils::factories::TestDataFactory;
use anyhow::Result;

pub struct LeadCreateRequestApi {
    client: &'static ApiClient,
}

impl LeadCreateRequestApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
        }
    }

    pub async fn create_lead(
        &self,
        country_code: String,
        lead_name: String,
        lead_phone: String,
        lead_email: String,
        lead_source_category: String,
        lead_source: String,
        campaign_name: String,
        lead_owner_uuid: String,
    ) -> Result<LeadCreateResponse> {
        let request = LeadCreateRequest {
            country_code,
            lead_name,
            lead_phone,
            lead_email,
            lead_source_category,
            lead_source,
            campaign_name,
            lead_owner_uuid,
        };
        let response = self
            .client
            .post("/api/v1/admin/add-temp-lead")
            .json(&request)
            .send_json::<LeadCreateResponse>()
            .await;

        response
    }

    pub async fn create_lead_with_factory(&self) -> Result<LeadCreateResponse> {
        let config = lib_test_helpers::config::get_config();
        self.create_lead(
            config.country_code.clone(),
            TestDataFactory::name(),
            TestDataFactory::phone(),
            TestDataFactory::email(),
            "test".to_string(),
            "rust_auto".to_string(),
            "test".to_string(),
            "".to_string(),
        )
        .await
    }
}

impl Default for LeadCreateRequestApi {
    fn default() -> Self {
        Self::new()
    }
}
