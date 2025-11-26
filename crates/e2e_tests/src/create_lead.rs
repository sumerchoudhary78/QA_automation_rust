use crate::login;
use crate::utils::drop_down_select::select_dropdown_option;
use lib_test_helpers::driver::global_driver;
use lib_test_helpers::session_file::{check_logged_in, restore_session};
use rand::Rng;
use std::time::Duration;
use thirtyfour::prelude::*;

pub async fn create_lead_test(base_url: &str) -> WebDriverResult<()> {
    let driver = global_driver().await?;
    let _ = restore_session(&driver, "session_daily", &base_url).await;
    let logg = check_logged_in(&driver, &base_url).await.unwrap_or(false);
    if !logg {
        let _ = login::login_test(&base_url).await;
    }
    tokio::time::sleep(Duration::from_millis(2000)).await;

    let mut rng = rand::thread_rng();
    let random_part_mobile = rng.gen_range(0..10_000_000);
    let random_part_name = rng.gen_range(0..100);
    let mobile_number = format!("444{:7}", random_part_mobile);
    let name = format!("Test_auto{:2}", random_part_name);

    driver
        .query(By::XPath("//a[contains(., 'Create Lead')]"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .click()
        .await?;

    driver
        .query(By::XPath("//input[@placeholder='Lead Name']"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .send_keys(&name)
        .await?;

    driver
        .query(By::XPath("//input[@placeholder='Lead Mobile Number']"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .send_keys(&mobile_number)
        .await?;

    driver
        .query(By::XPath("//input[@placeholder='Email Address']"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .send_keys("test_auto_1@gmail.com")
        .await?;

    select_dropdown_option(&driver, By::Id("basic_lead_source_category"), "Paid").await?;
    select_dropdown_option(&driver, By::Id("basic_lead_source"), "Website").await?;

    driver
        .query(By::XPath("//input[@placeholder='Campaign Name']"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?
        .send_keys("Test_WMS")
        .await?;

    select_dropdown_option(&driver, By::Id("basic_lead_owner_uuid"), "Sales Test").await?;

    driver
        .find(By::Css("button[type='submit']"))
        .await?
        .click()
        .await?;

    println!("✅ Create lead test passed!");
    Ok(())
}
