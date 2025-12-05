use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ElderDataEditRequest {
    pub currentElderIdentifier: String,
    pub consumeraddress_uuid: String,
    pub phone_no: String,
    pub is_nok_elder_calling: String,
    pub email: String,
    pub reasons_for_downloading_app: String,
    pub first_name: String,
    pub age: String,
    pub languages: String,
    pub last_name: String,
    pub gender: i16,
    pub current_medical_conditions: String,
    pub pincode: String,
    pub address_line_1: String,
    pub address_line_2: String,
    pub city: String,
    pub state: String,
    pub locality: String,
    pub country: String,
    pub new_selected_ep_uuid: String,
    pub r#override: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElderDataEditResponse {
    pub data: ElderDataEditResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElderDataEditResponseData {
    pub status: String,
}
