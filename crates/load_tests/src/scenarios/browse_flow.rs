use super::auth_flow::AuthToken;
use goose::prelude::*;

pub async fn get_current_user_task(user: &mut GooseUser) -> TransactionResult {
    let goose_request = if let Some(auth_token) = user.get_session_data::<AuthToken>() {
        GooseRequest::builder()
            .path("/api/v1/users/current-user")
            .set_request_builder(
                reqwest::Client::new()
                    .get(&format!("{}/api/v1/users/current-user", user.base_url))
                    .bearer_auth(&auth_token.0),
            )
            .build()
    } else {
        GooseRequest::builder()
            .path("/api/v1/users/current-user")
            .build()
    };

    let _response = user.request(goose_request).await?;
    Ok(())
}

pub async fn browse_leads_task(user: &mut GooseUser) -> TransactionResult {
    let goose_request = if let Some(auth_token) = user.get_session_data::<AuthToken>() {
        GooseRequest::builder()
            .path("/api/v1/admin/leads")
            .set_request_builder(
                reqwest::Client::new()
                    .get(&format!("{}/api/v1/admin/leads", user.base_url))
                    .bearer_auth(&auth_token.0),
            )
            .build()
    } else {
        GooseRequest::builder().path("/api/v1/admin/leads").build()
    };

    let _response = user.request(goose_request).await?;
    Ok(())
}

pub fn browse_scenario() -> Scenario {
    scenario!("Browse Flow")
        .register_transaction(
            transaction!(get_current_user_task)
                .set_name("Get Current User")
                .set_weight(3)
                .expect("Failed to set weight"),
        )
        .register_transaction(
            transaction!(browse_leads_task)
                .set_name("Browse Leads")
                .set_weight(2)
                .expect("Failed to set weight"),
        )
}
