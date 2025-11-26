use std::time::Duration;
use thirtyfour::prelude::*;

pub async fn select_dropdown_option(
    driver: &WebDriver,
    dropdown_selector: By,
    option_text: &str,
) -> WebDriverResult<()> {
    let dropdown = driver
        .query(dropdown_selector)
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .first()
        .await?;

    dropdown.scroll_into_view().await?;
    dropdown.click().await?;

    // let visible_dropdown = driver
    //     .query(By::Css(".ant-select-dropdown:not(.ant-select-dropdown-hidden)"))
    //     .wait(Duration::from_secs(10), Duration::from_millis(500))
    //     .first()
    //     .await?;

    let options = driver
        .query(By::Css(
            ".ant-select-dropdown:not(.ant-select-dropdown-hidden) .ant-select-item-option-content",
        ))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .all_from_selector()
        .await?;

    tokio::time::sleep(Duration::from_millis(300)).await;

    println!("Found {} visible dropdown options:", options.len());
    for opt in &options {
        let txt = opt.text().await.unwrap_or_default();
        let inner = opt
            .attr("innerHTML")
            .await
            .unwrap_or_default()
            .unwrap_or_default();
        println!(" → text: '{}', innerHTML: '{}'", txt.trim(), inner.trim());
    }

    for option in options {
        let text = option.text().await.unwrap_or_default();
        let inner = option
            .attr("innerHTML")
            .await
            .unwrap_or_default()
            .unwrap_or_default();

        if text.trim().eq_ignore_ascii_case(option_text.trim())
            || inner.trim().eq_ignore_ascii_case(option_text.trim())
        {
            option.scroll_into_view().await?;
            option.click().await?;
            println!("✅ Selected option: '{}'", option_text.trim());
            return Ok(());
        }
    }

    Err(WebDriverError::NotFound(
        format!("Dropdown option '{}' not found", option_text),
        String::from("None"),
    ))
}
