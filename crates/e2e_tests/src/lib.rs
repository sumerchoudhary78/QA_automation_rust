pub mod login;
pub mod create_lead;
pub mod create_invoice;
pub mod utils {
    pub mod drop_down_select;
    pub mod dashboard_button;
    pub mod retry;
}
use anyhow::Result;


pub async fn run() -> Result<()> {
    let cfg = lib_test_helpers::config::get_config();
    login::login_test(&cfg.base_url.as_str()).await?;
    create_invoice::create_invoice(&cfg.base_url.as_str()).await?;
    Ok(())
}
