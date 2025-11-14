use lib_test_helpers::driver::global_driver;
use thirtyfour::prelude::*;
use crate::utils::dashboard_button::select_dashboard;

pub async fn create_invoice(base_url: &str) -> WebDriverResult<()> {
    let driver = global_driver().await?;
    driver.goto(base_url).await?;

    select_dashboard(driver, By::XPath("//a[contains(., 'elders')]")).await?;
    Ok(())
}