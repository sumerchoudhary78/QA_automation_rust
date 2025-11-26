use crate::utils::dashboard_button::select_dashboard;
use lib_test_helpers::session_file::restore_session;
use thirtyfour::prelude::*;

pub async fn create_invoice(driver: WebDriver, base_url: String) -> WebDriverResult<()> {
    let _ = restore_session(&driver, "session_daily", &base_url).await;
    // driver.goto(&base_url).await?;
    let cfg = lib_test_helpers::config::get_config();

    select_dashboard(
        &driver,
        By::XPath("/html/body/div[1]/div[4]/div/div/aside/div/ul/li[6]/a"),
    )
    .await?;
    driver.goto(format!("/details{}", cfg.elder_number)).await?;

    Ok(())
}
