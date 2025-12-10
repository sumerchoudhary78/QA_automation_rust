use crate::client::ApiClient;
use crate::request_response::auth::{
    LoginInitResponse, LoginRequest, LoginResponse, OtpRequest, VerifyNumberSendOtpRequest,
};
use anyhow::{Context, Result};

pub struct AuthApi {
    client: &'static ApiClient,
}

impl AuthApi {
    pub fn new() -> Self {
        Self {
            client: ApiClient::global(),
        }
    }

    pub async fn login(&self, email: String, password: String) -> Result<LoginInitResponse> {
        let request = LoginRequest { email, password };

        let response = self
            .client
            .post("/api/v1/admin/login-new")
            .json(&request)
            .send_json::<LoginInitResponse>()
            .await
            .context("Login request failed")?;
        Ok(response)
    }

    pub async fn login_with_config(&self) -> Result<LoginInitResponse> {
        let config = lib_test_helpers::config::get_config();
        self.login(config.email.clone(), config.password.clone())
            .await
    }

    pub async fn verify_send_otp(
        &self,
        mobile_number: String,
        country_code: String,
        email: String,
    ) -> Result<()> {
        let request = VerifyNumberSendOtpRequest {
            mobile_number,
            country_code,
            email,
        };
        println!("Request: {:#?}", request);
        self.client
            .post("/api/v1/admin/verify-number-send-otp")
            .json(&request)
            .send()
            .await
            .context("OTP send failed")?;

        Ok(())
    }

    pub async fn verify_send_otp_with_config(&self) -> Result<()> {
        let config = lib_test_helpers::config::get_config();
        self.verify_send_otp(
            config.mobile_number.clone(),
            config.country_code.clone(),
            config.email.clone(),
        )
        .await
    }

    pub async fn verify_otp(
        &self,
        country_code: String,
        mobile_number: String,
        otp: String,
        email: String,
    ) -> Result<LoginResponse> {
        let request = OtpRequest {
            country_code,
            mobile_number,
            otp,
            email,
        };
        println!("Request: {:#?}", request);
        let response = self
            .client
            .post("api/v1/admin/verify-admin-login-otp")
            .json(&request)
            .send_json::<LoginResponse>()
            .await
            .context("OTP verification failed")?;
        self.client
            .set_auth_token(response.data.access_token.clone())
            .await;
        Ok(response)
    }

    pub async fn verify_otp_with_config(&self) -> Result<LoginResponse> {
        let config = lib_test_helpers::config::get_config();
        self.verify_otp(
            config.country_code.clone(),
            config.mobile_number.clone(),
            config.otp.clone(),
            config.email.clone(),
        )
        .await
    }

    pub async fn is_authenticated(&self) -> bool {
        self.client.get_auth_token().await.is_some()
    }

    pub async fn get_token(&self) -> Option<String> {
        self.client.get_auth_token().await
    }
}

impl Default for AuthApi {
    fn default() -> Self {
        Self::new()
    }
}
