use thirtyfour::prelude::*;
use std::{any::Any, time::Duration};
use crate::create_lead;

pub async fn login_test() -> WebDriverResult<()> {
    // Load config (if you have one)
    let base_url = "https://pataka.wms.emoha.com/";
    let browser = std::env::var("BROWSER").unwrap_or_else(|_| "chrome".to_string());

    // Create driver dynamically
    let driver = match browser.as_str() {
        "firefox" => {
            let caps = DesiredCapabilities::firefox();
            WebDriver::new("http://localhost:9515", caps).await?
        }
        _ => {
            let caps = DesiredCapabilities::chrome();
            WebDriver::new("http://localhost:9515", caps).await?
        }
    };

    // Go to base URL
    driver.goto(base_url).await?;

    // Example actions
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
    driver
        .query(By::XPath("//input[@placeholder='Enter Phone..']"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
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

    create_lead::create_lead_test(&driver).await?;
    println!("✅ Login test passed!");
    // driver.quit().await?;
    Ok(())
}
