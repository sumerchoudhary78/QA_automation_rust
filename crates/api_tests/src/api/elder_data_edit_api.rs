use io::BufRead;
use std::fs::File;
use std::io;
use std::ptr::null;

use crate::client::ApiClient;
use crate::request_response::elder_data_edit::ElderDataEditRequest;
use crate::request_response::elder_data_edit::ElderDataEditResponse;
use anyhow::Result;

pub struct ElderDataEditApi {
    client: &'static ApiClient,
}

impl ElderDataEditApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
        }
    }

    pub async fn update_elder_data(
        &self,
        request: ElderDataEditRequest,
    ) -> Result<ElderDataEditResponse> {
        let response = self
            .client
            .put("api/v1/admin/sales/elder/lead-profile/detail-of-person-calling")
            .json(&request)
            .send_json::<ElderDataEditResponse>()
            .await;

        response
    }

    pub async fn update_elder_data_with_factory(&self) -> Result<ElderDataEditResponse> {
        let path = "crates/api_tests/docs/profile_uuid.txt";
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let uuid = reader.lines().filter_map(Result::ok).last().unwrap();

        let path_profile_uuid = "crates/api_tests/docs/lead_uuid.txt";
        let file = File::open(path_profile_uuid).unwrap();
        let reader = io::BufReader::new(file);
        let profile_uuid = reader.lines().filter_map(Result::ok).last().unwrap();
        self.update_elder_data(ElderDataEditRequest {
            currentElderIdentifier: uuid.to_string(),
            consumeraddress_uuid: "".to_string(),
            phone_no: "1234567890".to_string(),
            is_nok_elder_calling: "false".to_string(),
            email: "test@example.com".to_string(),
            reasons_for_downloading_app: "test".to_string(),
            first_name: "test".to_string(),
            age: "test".to_string(),
            languages: "test".to_string(),
            last_name: "test".to_string(),
            gender: 1,
            current_medical_conditions: "test".to_string(),
            pincode: "test".to_string(),
            address_line_1: "test".to_string(),
            address_line_2: "test".to_string(),
            city: "test".to_string(),
            state: "test".to_string(),
            locality: "test".to_string(),
            country: "test".to_string(),
            new_selected_ep_uuid: "test".to_string(),
            r#override: "test".to_string(),
        })
        .await
    }
}
