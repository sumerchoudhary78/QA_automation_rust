use thirtyfour::prelude::*;
use std::time::Duration;


pub async fn retry<F, Fut, T>(mut action: F, retries: u32, delay_ms: u64) -> WebDriverResult<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = WebDriverResult<T>>,
{
    let mut last_err: Option<WebDriverError> = None;

    for _ in 0..retries {
        match action().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                last_err = Some(e);
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }

    Err(last_err.unwrap())
}
