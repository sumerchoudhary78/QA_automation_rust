use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
pub struct LeadCreateRequest {
    pub country_code: String,
    pub lead_name: String,
    pub lead_phone: String,
    pub lead_email: String,
    pub lead_source_category: String,
    pub lead_source: String,
    pub campaign_name: String,
    pub lead_owner_uuid: String,
}

#[derive(Deserialize)]
pub struct LeadCreateResponse {
    pub data: LeadCreateResponseData,
}

#[derive(Deserialize)]
pub struct LeadCreateResponseData {
    pub userName: String,
    pub userId: String,
    pub createdLeadDetails: CreatedLeadDetails,
}

#[derive(Deserialize)]
pub struct CreatedLeadDetails {
    pub uuid: String,
    pub id: i32,
}
