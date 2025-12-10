pub mod api;
pub mod api_context;
pub mod client;
pub mod models;
pub mod request_response;
pub mod utils;
pub use api::auth_api::AuthApi;
pub use api::create_invoice::CreateInvoiceApi;
pub use api::current_user::CurrentUserApi;
pub use api::elder_data_edit_api::ElderDataEditApi;
pub use api::lead_create::LeadCreateRequestApi;
pub use api::lead_stage_update::LeadStageUpdateApi;
pub use client::ApiClient;
pub use models::*;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};
pub use utils::*;

use anyhow::Result;

pub async fn run_all_tests() -> Result<()> {
    println!("api testing started\n");

    let auth = AuthApi::new();
    let lead_create = LeadCreateRequestApi::new();
    let lead_stage_update = LeadStageUpdateApi::new();
    let current_user = CurrentUserApi::new();
    let elder_data_edit = ElderDataEditApi::new();
    // let create_invoice = CreateInvoiceApi::new();

    println!("auth testing started\n");

    println!("step 1 login");
    match auth.login_with_config().await {
        Ok(response) => {
            println!("success step 1");
            println!(
                "Mobile: +{} {}",
                response.data.country_code, response.data.mobile_number
            );
        }
        Err(e) => {
            println!("failed step 1");
            eprintln!("Error details: {:?}", e);
            eprintln!("Full error chain:");
            let mut err: &dyn std::error::Error = &*e;
            loop {
                eprintln!("- {}", err);
                match err.source() {
                    Some(source) => err = source,
                    None => break,
                }
            }
            return Ok(());
        }
    }

    println!("step 2 send otp");
    match auth.verify_send_otp_with_config().await {
        Ok(()) => {
            println!("success step 2");
        }
        Err(e) => {
            println!("failed step 2");
            eprintln!("Error details: {:?}", e);
            eprintln!("Full error chain:");
            let mut err: &dyn std::error::Error = &*e;
            loop {
                eprintln!("- {}", err);
                match err.source() {
                    Some(source) => err = source,
                    None => break,
                }
            }
            return Ok(());
        }
    }

    println!("step 3 verify otp");
    match auth.verify_otp_with_config().await {
        Ok(response) => {
            println!("success step 3");
            println!(
                "token: {}...",
                &response.data.access_token[..20.min(response.data.access_token.len())]
            );
        }
        Err(e) => {
            println!("failed step 3");
            eprintln!("error details: {:?}", e);
            eprintln!("full error chain:");
            let mut err: &dyn std::error::Error = &*e;
            loop {
                eprintln!("- {}", err);
                match err.source() {
                    Some(source) => err = source,
                    None => break,
                }
            }
            return Ok(());
        }
    }

    match lead_create.create_lead_with_factory().await {
        Ok(response) => {
            let path = "crates/api_tests/docs/lead_uuid.txt";
            let path_profile_uuid = "crates/api_tests/docs/profile_uuid.txt";
            let profile_uuid_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path_profile_uuid)
                .unwrap();
            let mut profile_uuid_writer = BufWriter::new(profile_uuid_file);
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap();
            let mut writer = BufWriter::new(file);
            println!("success step 4");
            println!("user details {}", response.data.userName);
            println!("user id {}", response.data.userId);
            println!(
                "created lead details {}",
                response.data.createdLeadDetails.uuid
            );
            writeln!(writer, "{}", response.data.createdLeadDetails.uuid).unwrap();
            writeln!(profile_uuid_writer, "{}", response.data.userId).unwrap();
            current_user.get_current_user(&response.data.userId).await?;
        }
        Err(e) => {
            println!("failed step 4");
            eprintln!("error details: {:?}", e);
            eprintln!("full error chain:");
            let mut err: &dyn std::error::Error = &*e;
            loop {
                eprintln!("- {}", err);
                match err.source() {
                    Some(source) => err = source,
                    None => break,
                }
            }
            return Ok(());
        }
    };

    match lead_stage_update.update_lead_stage_with_factory().await {
        Ok(response) => {
            println!("success step 5");
            println!("response: {}", response.data);
        }
        Err(e) => {
            println!("failed step 5");
            eprintln!("error details: {:?}", e);
            eprintln!("full error chain:");
            let mut err: &dyn std::error::Error = &*e;
            loop {
                eprintln!("- {}", err);
                match err.source() {
                    Some(source) => err = source,
                    None => break,
                }
            }
            return Ok(());
        }
    }

    match elder_data_edit.update_elder_data_with_factory().await {
        Ok(response) => {
            println!("success step 6");
            println!("response: {:#?}", response);
        }
        Err(e) => {
            println!("failed step 6");
            eprintln!("error details: {:?}", e);
            eprintln!("full error chain:");
            let mut err: &dyn std::error::Error = &*e;
            loop {
                eprintln!("- {}", err);
                match err.source() {
                    Some(source) => err = source,
                    None => break,
                }
            }
            return Ok(());
        }
    }

    println!("success");

    Ok(())
}
