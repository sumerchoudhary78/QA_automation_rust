use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ElderDataEditRequest {
    pub currentElderIdentifier: String,
    pub consumeraddress_uuid: String,
    pub phone_no: String,
    pub is_nok_elder_calling: String,
    pub email: String,
    pub reasons_for_downloading_app: Option<String>,
    pub first_name: String,
    pub age: Option<i8>,
    pub languages: Option<String>,
    pub last_name: String,
    pub gender: i16,
    pub current_medical_conditions: Option<String>,
    pub pincode: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub country: Option<String>,
    pub new_selected_ep_uuid: Option<String>,
    pub r#override: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElderDataEditResponse {
    pub data: ElderDataEditResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElderDataEditResponseData {
    pub status: String,
}
