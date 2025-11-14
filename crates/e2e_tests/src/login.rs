use thirtyfour::prelude::*;
use std::{time::Duration};
use lib_test_helpers::{driver::global_driver, session_file::save_session};
use crate::utils::retry::retry;

pub async fn login_test(base_url:&str) -> WebDriverResult<()> {
    let driver = global_driver().await?;
    driver.goto(base_url).await?;

    driver
        .find(By::Id("basic_email"))
        .await?
        .send_keys("wmstest@emoha.com")
        .await?;
    driver
        .find(By::Id("basic_password"))
        .await?
        .send_keys("123456")
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
            .send_keys("9501285590")
            .await
        },
        5,
        400,
    ).await?;

    driver
        .query(By::XPath("//input[@placeholder='Enter Phone..']"))
        .wait(Duration::from_secs(50), Duration::from_millis(500))
        .first()
        .await?
        .send_keys("9501285590")
        .await?;
    driver
        .query(By::Css("button.otp-send-btn"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .click().await.ok();   

    driver
        .query(By::XPath("//input[@placeholder='Enter 4-digit OTP' and not(@disabled)]"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
    .send_keys("0180")
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

    tokio::time::sleep(Duration::from_millis(500)).await;
    let _ =save_session(&driver, "session_daily").await;
    
    // create_lead::create_lead_test(&driver).await?;

    println!("✅ Login test passed!");
    // driver.quit().await?;
    Ok(())
}
