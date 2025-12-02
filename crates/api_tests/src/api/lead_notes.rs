use io::BufRead;
use std::fs::File;
use std::io;

use crate::client::ApiClient;
use crate::factories;
use crate::request::lead_notes_request::LeadNotesAddRequest;
use crate::request::lead_notes_request::LeadNotesAddResponse;
use crate::request::lead_notes_request::LeadNotesGetResponse;
use anyhow::Result;

pub struct LeadNotesApi {
    client: &'static ApiClient,
}

impl LeadNotesApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
        }
    }

    pub async fn lead_notes_add(
        &self,
        request: LeadNotesAddRequest,
    ) -> Result<LeadNotesAddResponse> {
        let response = self
            .client
            .post("/api/v1/admin/sales/sales-member/lead-notes")
            .json(&request)
            .send_json::<LeadNotesAddResponse>()
            .await;

        response
    }

    pub async fn lead_notes_add_with_factory(&self) -> Result<LeadNotesAddResponse> {
        let path = "crates/api_tests/docs/lead_uuid.txt";
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let uuid = reader.lines().filter_map(Result::ok).last().unwrap();
        self.lead_notes_add(LeadNotesAddRequest {
            lead_uuid: uuid.to_string(),
            tag: "call".to_string(),
            notes: factories::TestDataFactory::random_string(20),
        })
        .await
    }

    pub async fn lead_notes_get(&self) -> Result<LeadNotesGetResponse> {
        let path = "crates/api_tests/docs/lead_uuid.txt";
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);
        let uuid = reader.lines().filter_map(Result::ok).last().unwrap();
        let response = self
            .client
            .get(&format!(
                "api/v1/admin/sales/elder/lead-notes?lead_uuid={}",
                uuid
            ))
            .send_json::<LeadNotesGetResponse>()
            .await;

        response
    }

    pub async fn lead_notes_get_with_factory(&self) -> Result<LeadNotesGetResponse> {
        let response = self.lead_notes_get().await;
        response
    }
}
