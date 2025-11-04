use clap::{Parser, Subcommand};
use anyhow::Result;
use lib_test_helpers::config::get_config;

/// Main CLI for QA Automation Runner
#[derive(Parser)]
#[command(name = "qa-runner", version, about = "QA Automation Orchestrator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run End-to-End Tests
    E2E,
    /// Run API Tests
    Api,
    /// Run Load Tests
    Load,
    /// Run all test suites
    All,

    Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::E2E => {
            println!("🧪 Running E2E tests...");
            e2e_tests::run().await?;
        }
        Commands::Api => {
            println!("🔗 Running API tests...");
            api_tests::ping();
        }
        Commands::Load => {
            println!("⚙️ Running load tests...");
            load_tests::run_load();
        }
        Commands::All => {
            println!("🚀 Running all tests...");
            api_tests::ping();
            e2e_tests::run();
            load_tests::run_load();
        }
        Commands::Config => {
            let cfg = get_config();
            println!("Current Config: {:#?}", cfg);
        }
    }

    Ok(())
}
