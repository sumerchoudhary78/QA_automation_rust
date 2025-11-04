pub mod login;
pub mod create_lead;
pub mod utils {
    pub mod drop_down_select;
}
use anyhow::Result;

/// Entry point for E2E test suite
pub async fn run() -> Result<()> {
    login::login_test().await?;
    Ok(())
}
