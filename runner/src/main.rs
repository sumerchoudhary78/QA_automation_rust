use anyhow::Result;
use clap::{Parser, Subcommand};
use lib_test_helpers::chrome_driver::chrome;
use lib_test_helpers::config::get_config;

#[derive(Parser)]
#[command(name = "qa-runner", version, about = "QA Automation Orchestrator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    E2E,
    Api,
    Load,
    All,
    Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::E2E => {
            println!("Running E2E tests...");
            let mut cd = chrome();
            e2e_tests::run().await?;
            // if let Err(e) = e2e_tests::run().await {
            //     let _ = cd.kill();
            //     return Err(e);
            // }
            cd.kill().expect("kill process");
        }
        Commands::Api => {
            println!("Running API tests...");
            api_tests::run_all_tests().await?;
        }
        Commands::Load => {
            println!("Running load tests...");
            load_tests::run_load();
        }
        Commands::All => {
            println!("Running all tests...");
            api_tests::run_all_tests().await?;
            e2e_tests::run().await?;
            load_tests::run_load();
        }
        Commands::Config => {
            let cfg = get_config();
            println!("Current Config: {:#?}", cfg);
        }
    }

    Ok(())
}
