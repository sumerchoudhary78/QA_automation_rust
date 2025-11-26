use std::time::Duration;

use lib_test_helpers::session_file::restore_session;
use thirtyfour::prelude::*;

use crate::utils::drop_down_select::{self, select_dropdown_option};

pub async fn create_invoice(driver: WebDriver, base_url: String) -> WebDriverResult<()> {
    let _ = restore_session(&driver, "session_daily", &base_url).await;
    // driver.goto(&base_url).await?;
    let cfg = lib_test_helpers::config::get_config();

    // select_dashboard(
    //     &driver,
    //     By::XPath("/html/body/div[1]/div[4]/div/div/aside/div/ul/li[6]/a"),
    // )
    // .await?;
    driver
        .goto(format!("{}elder/details/{}", base_url, cfg.elder_number))
        .await?;

    tokio::time::sleep(Duration::from_secs(15)).await;
    driver
        .query(By::XPath(
            r#"//*[@id="app"]/div[5]/main/div[1]/div[2]/div/button[1]"#,
        ))
        .first()
        .await?
        .click()
        .await?;

    drop_down_select::select_dropdown_option(
        &driver,
        By::XPath("//*[@id=\"billing-center-select\"]"),
        "Testing Node",
    )
    .await?;

    driver
        .find(By::XPath(
            r#"//*[@id="app"]/div[5]/div/main/div[1]/div[1]/div[1]/div/button"#,
        ))
        .await?
        .click()
        .await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    driver
        .find(By::Css("button.ant-btn.ant-btn-round.ant-btn-primary"))
        .await?
        .click()
        .await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    select_dropdown_option(
        &driver,
        By::Css("div.ant-select-selector span.ant-select-selection-search"),
        "HR,Haryana",
    )
    .await?;
    Ok(())
}
