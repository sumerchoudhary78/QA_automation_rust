use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub data: Option<T>,
    pub meta: Option<ApiError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn is_success(&self) -> bool {
        self.code == 200 && self.meta.is_none()
    }

    pub fn into_result(self) -> Result<T, ApiError> {
        if let Some(data) = self.data {
            Ok(data)
        } else if let Some(error) = self.meta {
            Err(error)
        } else {
            Err(ApiError {
                code: 500,
                message: "No data or error in response".to_string(),
            })
        }
    }
}
