use crate::utils::dashboard_button::select_dashboard;
use lib_test_helpers::session_file::restore_session;
use thirtyfour::prelude::*;

pub async fn create_invoice(driver: WebDriver, base_url: String) -> WebDriverResult<()> {
    let _ = restore_session(&driver, "session_daily", &base_url).await;

    let url = format!("{}/elders", base_url);
    driver.goto(&url).await?;
    select_dashboard(&driver, By::XPath("//a[contains(., 'elders')]")).await?;
    Ok(())
}
