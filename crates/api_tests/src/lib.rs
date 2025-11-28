pub mod auth;
pub mod client;

pub mod models;

pub mod api;

pub mod utils;
pub use api::AuthApi;
pub use client::ApiClient;
pub use models::*;
pub use utils::*;

use anyhow::Result;
pub async fn run_all_tests() -> Result<()> {
    println!("api testing started\n");

    let auth = AuthApi::new();

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
    println!("success");

    Ok(())
}
