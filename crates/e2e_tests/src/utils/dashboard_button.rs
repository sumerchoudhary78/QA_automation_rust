use thirtyfour::{ prelude::*};
use std::time::Duration;

pub async  fn select_dashboard(driver:&WebDriver, dashboard_selector:By) -> WebDriverResult<()> { 


    let scroll = driver
    .query(dashboard_selector.clone())
    .wait(Duration::from_secs(5), Duration::from_millis(300))
    .first()
    .await?;
    scroll.scroll_into_view().await?;
    scroll.click().await?;
    driver
    .query(dashboard_selector)
    .wait(Duration::from_secs(5), Duration::from_millis(300))
    .first()
    .await?
    .click()
    .await?;

    Ok(())

}