pub mod client;
pub mod models;
pub mod request;

pub mod api;

pub mod utils;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use api::auth_api::AuthApi;
pub use client::ApiClient;
pub use models::*;
pub use utils::*;

use anyhow::Result;

use crate::api::{lead_create::LeadCreateRequestApi, lead_stage_update::LeadStageUpdateApi};

pub async fn run_all_tests() -> Result<()> {
    println!("api testing started\n");

    let auth = AuthApi::new();
    let lead_create = LeadCreateRequestApi::new();
    let lead_stage_update = LeadStageUpdateApi::new();

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
    }

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
    println!("success");

    Ok(())
}
