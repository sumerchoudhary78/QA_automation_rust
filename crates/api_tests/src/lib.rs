pub mod api;
pub mod api_context;
pub mod client;
pub mod models;
pub mod reporting;
pub mod request_response;
pub mod utils;
pub use api::auth_api::AuthApi;
pub use api::create_invoice::CreateInvoiceApi;
pub use api::current_user::CurrentUserApi;
pub use api::elder_data_edit_api::ElderDataEditApi;
pub use api::lead_create::LeadCreateRequestApi;
pub use api::lead_stage_update::LeadStageUpdateApi;
pub use api::Invoices::pos_checkout::PosCheckoutApi;
pub use api::Invoices::pos_items_all::PosItemsAllApi;
pub use api::Invoices::update_pos_cart_item::UpdatePosCartItemApi;
pub use api::Invoices::void_invoice::VoidInvoiceApi;
pub use client::ApiClient;
use futures::FutureExt;
pub use models::*;
use std::panic::AssertUnwindSafe;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};
pub use utils::*;

use anyhow::Result;
use reporting::{
    ConsoleReporter, HtmlReporter, JUnitReporter, JsonReporter, Reporter, TestCollector,
};
use std::path::Path;
use std::time::Instant;

fn generate_reports() -> Result<()> {
    let collector = TestCollector::global();
    let report = collector.generate_report();

    ConsoleReporter::new().print(&report);

    let reports_dir = Path::new("crates/api_tests/reports");
    std::fs::create_dir_all(reports_dir)?;
    let html_path = reports_dir.join("api_report.html");
    HtmlReporter::new().save(&report, &html_path)?;
    println!("HTML Report: {}", html_path.display());

    let json_path = reports_dir.join("api_report.json");
    JsonReporter::new().save(&report, &json_path)?;
    println!("JSON Report: {}", json_path.display());
    let junit_path = reports_dir.join("api_report.xml");
    JUnitReporter::new().save(&report, &junit_path)?;
    println!("JUnit Report: {}", junit_path.display());

    println!("All reports generated successfully");

    if report.has_failures() {
        println!(
            "{} test(s) failed. Check reports for details.",
            report.failed
        );
    }

    Ok(())
}

pub async fn run_all_tests() -> Result<()> {
    println!("API Testing Started");

    let collector = TestCollector::global();
    collector.clear();
    collector.start();
    collector.set_suite_name("Emoha API Test Suite");

    let test_result = AssertUnwindSafe(run_test_steps()).catch_unwind().await;

    println!("Generating Reports...");
    if let Err(e) = generate_reports() {
        eprintln!("Failed to generate reports: {}", e);
    }

    match test_result {
        Ok(result) => result,
        Err(panic_info) => {
            let msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            Err(anyhow::anyhow!("Test execution panicked: {}", msg))
        }
    }
}

async fn run_test_steps() -> Result<()> {
    let collector = TestCollector::global();

    let auth = AuthApi::new();
    let lead_create = LeadCreateRequestApi::new();
    let lead_stage_update = LeadStageUpdateApi::new();
    let current_user = CurrentUserApi::new();
    let elder_data_edit = ElderDataEditApi::new();
    let get_pos_items_all = PosItemsAllApi::new();
    let update_pos_cart_item = UpdatePosCartItemApi::new();
    let pos_checkout = PosCheckoutApi::new();
    let void_invoice = VoidInvoiceApi::new();

    let mut should_continue = true;

    {
        let start = Instant::now();
        match auth.login_with_config().await {
            Ok(response) => {
                collector.record_passed(
                    "Step 1 - Login",
                    "Login API",
                    "POST",
                    "/api/v1/auth/login",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!(
                    "Login successful - Mobile: +{} {}",
                    response.data.country_code, response.data.mobile_number
                );
            }
            Err(e) => {
                collector.record_failed(
                    "Step 1 - Login",
                    "Login API",
                    "POST",
                    "/api/v1/auth/login",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Login failed - {}", e);
                should_continue = false;
            }
        }
    }

    if should_continue {
        let start = Instant::now();
        match auth.verify_send_otp_with_config().await {
            Ok(()) => {
                collector.record_passed(
                    "Step 2 - Send OTP",
                    "Send OTP API",
                    "POST",
                    "/api/v1/auth/send-otp",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!("Send OTP successful");
            }
            Err(e) => {
                collector.record_failed(
                    "Step 2 - Send OTP",
                    "Send OTP API",
                    "POST",
                    "/api/v1/auth/send-otp",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Send OTP failed - {}", e);
                should_continue = false;
            }
        }
    }

    if should_continue {
        let start = Instant::now();
        match auth.verify_otp_with_config().await {
            Ok(response) => {
                collector.record_passed(
                    "Step 3 - Verify OTP",
                    "Verify OTP API",
                    "POST",
                    "/api/v1/auth/verify-otp",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!(
                    "Verify OTP successful - token: {}...",
                    &response.data.access_token[..20.min(response.data.access_token.len())]
                );
            }
            Err(e) => {
                collector.record_failed(
                    "Step 3 - Verify OTP",
                    "Verify OTP API",
                    "POST",
                    "/api/v1/auth/verify-otp",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Verify OTP failed - {}", e);
                should_continue = false;
            }
        }
    }

    if should_continue {
        let start = Instant::now();
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
                writeln!(writer, "{}", response.data.createdLeadDetails.uuid).unwrap();
                writeln!(profile_uuid_writer, "{}", response.data.userId).unwrap();
                let _ = current_user.get_current_user(&response.data.userId).await;

                collector.record_passed(
                    "Step 4 - Create Lead",
                    "Create Lead API",
                    "POST",
                    "/api/v1/admin/leads",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!(
                    "Create Lead successful - User: {}, Lead UUID: {}",
                    response.data.userName, response.data.createdLeadDetails.uuid
                );
            }
            Err(e) => {
                collector.record_failed(
                    "Step 4 - Create Lead",
                    "Create Lead API",
                    "POST",
                    "/api/v1/admin/leads",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Create Lead failed - {}", e);
                should_continue = true;
            }
        }
    }

    if should_continue {
        let start = Instant::now();
        match lead_stage_update.update_lead_stage_with_factory().await {
            Ok(response) => {
                collector.record_passed(
                    "Step 5 - Update Lead Stage",
                    "Update Lead Stage API",
                    "PUT",
                    "/api/v1/admin/leads/stage",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!("Update Lead Stage successful - {}", response.data);
            }
            Err(e) => {
                collector.record_failed(
                    "Step 5 - Update Lead Stage",
                    "Update Lead Stage API",
                    "PUT",
                    "/api/v1/admin/leads/stage",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Update Lead Stage failed - {}", e);
            }
        }
    }

    if should_continue {
        let start = Instant::now();
        match elder_data_edit.update_elder_data_with_factory().await {
            Ok(_response) => {
                collector.record_passed(
                    "Step 6 - Update Elder Data",
                    "Update Elder Data API",
                    "PUT",
                    "/api/v1/admin/elder-data",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!("Update Elder Data successful");
            }
            Err(e) => {
                collector.record_failed(
                    "Step 6 - Update Elder Data",
                    "Update Elder Data API",
                    "PUT",
                    "/api/v1/admin/elder-data",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Update Elder Data failed - {}", e);
            }
        }
    }

    if should_continue {
        let start = Instant::now();
        match get_pos_items_all
            .get_pos_items_all("ea5639b8-0fe6-4676-b455-65e251cbc492")
            .await
        {
            Ok(()) => {
                collector.record_passed(
                    "Step 7 - Get POS Items",
                    "Get POS Items API",
                    "GET",
                    "/api/v1/admin/pos-items-all",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!("Get POS Items successful");
            }
            Err(e) => {
                collector.record_failed(
                    "Step 7 - Get POS Items",
                    "Get POS Items API",
                    "GET",
                    "/api/v1/admin/pos-items-all",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Get POS Items failed - {}", e);
            }
        }
    }

    {
        let start = Instant::now();
        match update_pos_cart_item
            .update_pos_cart_item_with_factory("ea5639b8-0fe6-4676-b455-65e251cbc492")
            .await
        {
            Ok(()) => {
                collector.record_passed(
                    "Step 8 - Update POS Cart Item",
                    "Update POS Cart Item API",
                    "PUT",
                    "/api/v1/admin/update-pos-cart-item-wms",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!("Update POS Cart Item successful");
            }
            Err(e) => {
                collector.record_failed(
                    "Step 8 - Update POS Cart Item",
                    "Update POS Cart Item API",
                    "PUT",
                    "/api/v1/admin/update-pos-cart-item-wms",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Update POS Cart Item failed - {}", e);
            }
        }
    }

    {
        let start = Instant::now();
        match update_pos_cart_item
            .update_pos_cart_item_with_factory_and_price(
                "ea5639b8-0fe6-4676-b455-65e251cbc492",
                &100.0,
            )
            .await
        {
            Ok(()) => {
                collector.record_passed(
                    "Step 9 - Update POS Cart with Price",
                    "Update POS Cart Item API",
                    "PUT",
                    "/api/v1/admin/update-pos-cart-item-wms",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!("Update POS Cart with Price successful");
            }
            Err(e) => {
                collector.record_failed(
                    "Step 9 - Update POS Cart with Price",
                    "Update POS Cart Item API",
                    "PUT",
                    "/api/v1/admin/update-pos-cart-item-wms",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("Update POS Cart with Price failed - {}", e);
            }
        }
    }

    {
        let start = Instant::now();
        match pos_checkout
            .pos_checkout_with_factory("ea5639b8-0fe6-4676-b455-65e251cbc492")
            .await
        {
            Ok(()) => {
                collector.record_passed(
                    "Step 10 - POS Checkout",
                    "POS Checkout API",
                    "POST",
                    "/api/v1/admin/pos-checkout",
                    200,
                    start.elapsed().as_millis() as u64,
                );
                println!("POS Checkout successful");

                let void_start = Instant::now();
                match void_invoice.void_invoice_with_factory().await {
                    Ok(()) => {
                        collector.record_passed(
                            "Step 11 - Void Invoice",
                            "Void Invoice API",
                            "PUT",
                            "/api/v1/admin/void-receipt",
                            200,
                            void_start.elapsed().as_millis() as u64,
                        );
                        println!("Void Invoice successful");
                    }
                    Err(e) => {
                        collector.record_failed(
                            "Step 11 - Void Invoice",
                            "Void Invoice API",
                            "PUT",
                            "/api/v1/admin/void-receipt",
                            None,
                            void_start.elapsed().as_millis() as u64,
                            e.to_string(),
                        );
                        println!("Void Invoice failed - {}", e);
                    }
                }
            }
            Err(e) => {
                collector.record_failed(
                    "Step 10 - POS Checkout",
                    "POS Checkout API",
                    "POST",
                    "/api/v1/admin/pos-checkout",
                    None,
                    start.elapsed().as_millis() as u64,
                    e.to_string(),
                );
                println!("POS Checkout failed - {}", e);
            }
        }
    }

    Ok(())
}
