use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LeadNotesAddRequest {
    pub lead_uuid: String,
    pub tag: String,
    pub notes: String,
}

#[derive(Deserialize)]
pub struct LeadNotesAddResponse {
    pub data: LeadNotesAddResponseData,
}

#[derive(Deserialize)]
pub struct LeadNotesAddResponseData {
    pub uuid: String,
}

#[derive(Deserialize)]
pub struct LeadNotesGetResponse {
    pub data: LeadNotesGetResponseData,
}

#[derive(Deserialize)]
pub struct LeadNotesGetResponseData {
    pub data: Vec<LeadNotesGetResponseDataItem>,
}

#[derive(Deserialize)]
pub struct LeadNotesGetResponseDataItem {
    pub engagement_creation_date: String,
    pub stage_id: u32,
    pub campaign_name: bool,
    pub source: bool,
    pub notes: String,
    pub tag: String,
    pub created_at: String,
    pub note_added_by: String,
}
