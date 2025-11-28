use anyhow::Result;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Create a new retry config with custom attempts
    pub fn with_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Create a new retry config with custom initial delay
    pub fn with_initial_delay(mut self, delay_ms: u64) -> Self {
        self.initial_delay_ms = delay_ms;
        self
    }

    /// Create a new retry config with custom max delay
    pub fn with_max_delay(mut self, delay_ms: u64) -> Self {
        self.max_delay_ms = delay_ms;
        self
    }
}

/// Retry an async operation with exponential backoff
pub async fn retry_async<F, Fut, T, E>(config: RetryConfig, mut operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    let mut delay_ms = config.initial_delay_ms;

    loop {
        attempt += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt >= config.max_attempts {
                    return Err(error);
                }

                sleep(Duration::from_millis(delay_ms)).await;

                // Calculate next delay with exponential backoff
                delay_ms =
                    ((delay_ms as f64 * config.backoff_multiplier) as u64).min(config.max_delay_ms);
            }
        }
    }
}

/// Retry with default configuration
pub async fn retry_default<F, Fut, T, E>(operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    retry_async(RetryConfig::default(), operation).await
}

/// Retry a specific number of times
pub async fn retry_times<F, Fut, T, E>(attempts: u32, operation: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let config = RetryConfig::default().with_attempts(attempts);
    retry_async(config, operation).await
}

/// Retry with condition checking
pub async fn retry_until<F, Fut, T, E, C>(
    config: RetryConfig,
    mut operation: F,
    mut should_retry: C,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    C: FnMut(&E) -> bool,
{
    let mut attempt = 0;
    let mut delay_ms = config.initial_delay_ms;

    loop {
        attempt += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt >= config.max_attempts || !should_retry(&error) {
                    return Err(error);
                }

                sleep(Duration::from_millis(delay_ms)).await;

                delay_ms =
                    ((delay_ms as f64 * config.backoff_multiplier) as u64).min(config.max_delay_ms);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_succeeds_on_first_attempt() {
        let config = RetryConfig::default();
        let result = retry_async(config, || async { Ok::<i32, String>(42) }).await;
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_retry_exhausts_attempts() {
        let config = RetryConfig::default().with_attempts(2);

        let result = retry_async(config, || async { Err::<i32, &str>("always fails") }).await;

        assert!(result.is_err());
    }

    #[test]
    fn test_retry_config_builder() {
        let config = RetryConfig::default()
            .with_attempts(5)
            .with_initial_delay(200)
            .with_max_delay(10000);

        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.initial_delay_ms, 200);
        assert_eq!(config.max_delay_ms, 10000);
    }
}
