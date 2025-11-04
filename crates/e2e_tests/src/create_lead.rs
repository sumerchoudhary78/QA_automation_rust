use thirtyfour::prelude::*;
use std::{any::Any, time::Duration};
use crate::utils::drop_down_select::select_dropdown_option;
use rand::Rng;

pub async fn create_lead_test(driver: &WebDriver) -> WebDriverResult<()> {

    let mut rng = rand::thread_rng();
    let random_part_mobile = rng.gen_range(0..10_000_000);
    let random_part_name = rng.gen_range(0..100);
    let mobile_number = format!("444{:7}",random_part_mobile);
    let name = format!("Test_auto{:2}",random_part_name);

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
    select_dropdown_option(&driver, By::Id("basic_lead_source"), "Website Chat").await?;

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
    .click().await?;

    println!("✅ Create lead test passed!");
    // driver.quit().await?;
    Ok(())
}       