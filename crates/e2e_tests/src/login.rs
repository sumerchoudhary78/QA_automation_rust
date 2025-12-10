use crate::utils::retry::retry;
use lib_test_helpers::config::get_config;
use lib_test_helpers::{driver::global_driver, session_file::save_session};
use std::time::Duration;
use thirtyfour::prelude::*;
pub async fn login_test(base_url: &str) -> WebDriverResult<()> {
    let driver = global_driver().await?;
    driver.goto(base_url).await?;
    let cfg = get_config();

    driver
        .find(By::Id("basic_email"))
        .await?
        .send_keys(&cfg.email)
        .await?;
    driver
        .find(By::Id("basic_password"))
        .await?
        .send_keys(&cfg.password)
        .await?;
    driver
        .find(By::Css("button[type='submit']"))
        .await?
        .click()
        .await?;

    retry(
        || async {
            driver
                .query(By::XPath("//input[@placeholder='Enter Phone..']"))
                .wait(Duration::from_secs(50), Duration::from_millis(500))
                .first()
                .await?
                .send_keys(&cfg.mobile_number)
                .await
        },
        5,
        400,
    )
    .await?;

    driver
        .query(By::XPath("//input[@placeholder='Enter Phone..']"))
        .wait(Duration::from_secs(50), Duration::from_millis(500))
        .first()
        .await?
        .send_keys(&cfg.mobile_number)
        .await?;
    driver
        .query(By::Css("button.otp-send-btn"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .click()
        .await
        .ok();

    driver
        .query(By::XPath(
            "//input[@placeholder='Enter 4-digit OTP' and not(@disabled)]",
        ))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .send_keys(&cfg.otp)
        .await?;
    driver
        .find(By::Css("button.enter-otp-verify-button-visible"))
        .await?
        .click()
        .await?;
    driver
        .query(By::Css("button.No.thanks"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//a[contains(., 'Create Lead')]"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?;

    let _ = save_session(&driver, "session_daily").await;

    // create_lead::create_lead_test(&driver).await?;

    println!("✅ Login test passed!");
    driver.quit().await?;
    Ok(())
}
