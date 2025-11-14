use thirtyfour::prelude::*;
use std::time::Duration;

pub async  fn select_dashboard(driver:WebDriver, dashboard_selector:By) -> WebDriverResult<()> { 

    driver
    .query(dashboard_selector)
    .wait(Duration::from_secs(5), Duration::from_millis(300))
    .first()
    .await?
    .click()
    .await?;

    Ok(())

}