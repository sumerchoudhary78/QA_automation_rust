use goose::prelude::*;

pub async fn login_task(user: &mut GooseUser) -> TransactionResult {
    let config = lib_test_helpers::config::get_config();

    let login_body = serde_json::json!({
        "email": config.email,
        "password": config.password
    });

    let _response = user
        .post_json("/api/v1/admin/login-new", &login_body)
        .await?;

    Ok(())
}

pub async fn send_otp_task(user: &mut GooseUser) -> TransactionResult {
    let config = lib_test_helpers::config::get_config();

    let otp_body = serde_json::json!({
        "mobile_number": config.mobile_number,
        "country_code": config.country_code,
        "email": config.email
    });

    let _response = user
        .post_json("/api/v1/admin/verify-number-send-otp", &otp_body)
        .await?;

    Ok(())
}

pub async fn verify_otp_task(user: &mut GooseUser) -> TransactionResult {
    let config = lib_test_helpers::config::get_config();

    let verify_body = serde_json::json!({
        "country_code": config.country_code,
        "mobile_number": config.mobile_number,
        "otp": config.otp,
        "email": config.email
    });

    let goose_response = user
        .post_json("/api/v1/admin/verify-admin-login-otp", &verify_body)
        .await?;

    if let Ok(response) = goose_response.response {
        if let Ok(json) = response.json::<serde_json::Value>().await {
            if let Some(token) = json
                .get("data")
                .and_then(|d| d.get("access_token"))
                .and_then(|t| t.as_str())
            {
                user.set_session_data(AuthToken(token.to_string()));
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct AuthToken(pub String);

pub fn auth_scenario() -> Scenario {
    scenario!("Authentication Flow")
        .register_transaction(transaction!(login_task).set_name("Login"))
        .register_transaction(transaction!(send_otp_task).set_name("Send OTP"))
        .register_transaction(transaction!(verify_otp_task).set_name("Verify OTP"))
}
