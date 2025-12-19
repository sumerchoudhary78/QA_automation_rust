use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LeadStageUpdateRequest {
    pub lead_uuid: String,
    pub lead_stage_1_uuid: String,
    pub lead_stage_2_uuid: String,
    pub lead_stage_3_uuid: String,
    pub lead_stage_4_uuid: String,
    pub lead_stage_5_uuid: String,
    pub lead_stage_6_uuid: String,
    pub service_uuid: String,
    pub elder_id: String,
    pub opportunity_type_name: String,
    pub opportunity_type: String,
    pub lead_stage_3_follow_up: String,
    pub lead_stage_3_remark: String,
}

#[derive(Deserialize)]
pub struct LeadStageUpdateResponse {
    pub data: bool,
}

#[derive(Deserialize, Debug)]
pub struct LeadStages {
    pub lead_stage_1_uuid: String,
    pub lead_stage_2_uuid: String,
    pub lead_stage_3_uuid: String,
    pub lead_stage_4_uuid: String,
    pub lead_stage_5_uuid: String,
    pub lead_stage_6_uuid: String,
}

#[derive(Deserialize)]
pub struct ServiceUuid {
    pub service_uuid: String,
}
