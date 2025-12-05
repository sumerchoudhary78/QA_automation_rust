use crate::client::ApiClient;
use crate::factories;
use crate::request_response::lead_stage_update_request::LeadStageUpdateRequest;
use crate::request_response::lead_stage_update_request::LeadStageUpdateResponse;
use crate::request_response::lead_stage_update_request::LeadStages;
use crate::request_response::lead_stage_update_request::ServiceUuid;
use anyhow::Result;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct LeadStageUpdateApi {
    client: &'static ApiClient,
}

impl LeadStageUpdateApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
        }
    }

    pub async fn update_lead_stage(
        &self,
        request: LeadStageUpdateRequest,
    ) -> Result<LeadStageUpdateResponse> {
        println!("request: {:#?}", request);
        let response = self
            .client
            .put("/api/v1/admin/sales/sales-member/update-lead-stage")
            .json(&request)
            .send_json::<LeadStageUpdateResponse>()
            .await;

        response
    }

    pub async fn update_lead_stage_with_factory(&self) -> Result<LeadStageUpdateResponse> {
        // let config = lib_test_helpers::config::get_config();
        let path = "crates/api_tests/docs/lead_uuid.txt";
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let uuid = reader.lines().filter_map(Result::ok).last().unwrap();

        let stage_uuid_path = "crates/api_tests/docs/lead_stage_uuid.json";
        let file_contents = fs::read_to_string(stage_uuid_path).unwrap();
        let stages: LeadStages = serde_json::from_str(&file_contents).unwrap();
        let service_uuid_path = "crates/api_tests/docs/service_type_uuid.json";
        let file_contents = fs::read_to_string(service_uuid_path).unwrap();
        let service_uuid: ServiceUuid = serde_json::from_str(&file_contents).unwrap();

        let path_profile_uuid = "crates/api_tests/docs/profile_uuid.txt";
        let file = File::open(path_profile_uuid).unwrap();
        let reader = io::BufReader::new(file);
        let profile_uuid = reader.lines().filter_map(Result::ok).last().unwrap();
        self.update_lead_stage(LeadStageUpdateRequest {
            lead_uuid: uuid.to_string(),
            lead_stage_1_uuid: stages.lead_stage_1_uuid.to_string(),
            lead_stage_2_uuid: stages.lead_stage_2_uuid.to_string(),
            lead_stage_3_uuid: stages.lead_stage_3_uuid.to_string(),
            lead_stage_4_uuid: stages.lead_stage_4_uuid.to_string(),
            lead_stage_5_uuid: stages.lead_stage_5_uuid.to_string(),
            lead_stage_6_uuid: stages.lead_stage_6_uuid.to_string(),
            service_uuid: service_uuid.service_uuid.clone(),
            elder_id: profile_uuid.clone(),
            opportunity_type_name: "Empower Care Plan".to_string(),
            opportunity_type: service_uuid.service_uuid.clone(),
            lead_stage_3_follow_up: factories::TestDataFactory::time_fake().to_string(),
            lead_stage_3_remark: "".to_string(),
        })
        .await
    }
}
