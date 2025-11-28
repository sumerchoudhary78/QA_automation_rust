use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: Option<String>,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn is_success(&self) -> bool {
        self.success && self.error.is_none()
    }

    pub fn into_result(self) -> Result<T, ApiError> {
        if let Some(data) = self.data {
            Ok(data)
        } else if let Some(error) = self.error {
            Err(error)
        } else {
            Err(ApiError {
                code: Some("UNKNOWN_ERROR".to_string()),
                message: "No data or error in response".to_string(),
                details: None,
            })
        }
    }
}

impl<T> PaginatedResponse<T> {
    pub fn has_next_page(&self) -> bool {
        self.page < self.total_pages
    }

    pub fn is_first_page(&self) -> bool {
        self.page == 1
    }

    pub fn next_page(&self) -> Option<u32> {
        if self.has_next_page() {
            Some(self.page + 1)
        } else {
            None
        }
    }
}
