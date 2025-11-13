use thirtyfour::prelude::*;
use dotenvy::dotenv;
use std::env;

pub async fn global_driver() -> WebDriverResult<WebDriver> {
    dotenv().ok();
    let browser = env::var("BROWSER").unwrap_or_else(|_| "chrome".to_string());
    let server_url = env::var("WEBDRIVER_URL").unwrap_or_else(|_| "http://localhost:9515".to_string());

    let driver = match browser.as_str() {
        "firefox" => {
            let caps = DesiredCapabilities::firefox();
            WebDriver::new(&server_url, caps).await?
        }
        _ => {
            let mut caps = DesiredCapabilities::chrome();
            caps.add_arg("--no-sandbox")?;
            caps.add_arg("--disable-dev-shm-usage")?;
            caps.add_arg("--disable-gpu")?;
            caps.add_arg("--disable-extensions")?;
            caps.add_arg("--disable-infobars")?;
            caps.add_arg("--remote-debugging-port=9222")?;
            WebDriver::new(&server_url, caps).await?
        }
    };

    Ok(driver)
}
