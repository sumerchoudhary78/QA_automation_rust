use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInitResponse {
    pub code: u16,
    pub data: LoginInitData,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct LoginInitData {
    pub mobile_number: String,
    pub country_code: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub code: u16,
    pub data: LoginResponseData,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponseData {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct OtpRequest {
    pub country_code: String,
    pub mobile_number: String,
    pub otp: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyNumberSendOtpRequest {
    pub mobile_number: String,
    pub country_code: String,
    pub email: String,
}
