pub mod create_invoice;
pub mod create_lead;
pub mod login;
pub mod utils {
    pub mod dashboard_button;
    pub mod drop_down_select;
    pub mod retry;
}

use anyhow::Result;

pub async fn run() -> Result<()> {
    let cfg = lib_test_helpers::config::get_config();
    // login::login_test(&cfg.base_url.as_str()).await?;
    // tokio::time::sleep(Duration::from_millis(2000)).await;
    // create_lead::create_lead_test(&cfg.base_url.as_str()).await?;
    create_invoice::create_invoice(
        lib_test_helpers::driver::global_driver().await?,
        cfg.base_url.clone(),
    )
    .await?;
    Ok(())
}
