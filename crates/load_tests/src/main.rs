use goose::prelude::*;
use load_tests::scenarios::{auth_flow, browse_flow};

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("goose=info".parse().unwrap())
                .add_directive("load_tests=debug".parse().unwrap()),
        )
        .init();

    println!("Starting Load Tests...");
    println!("Available scenarios:");
    println!("  1. Authentication Flow - Login -> OTP -> Verify");
    println!("  2. Browse Flow - Read operations (current user, leads)");
    println!();

    GooseAttack::initialize()?
        .register_scenario(auth_flow::auth_scenario().set_weight(1)?)
        .register_scenario(browse_flow::browse_scenario().set_weight(3)?)
        .execute()
        .await?;

    println!("Load tests completed!");

    Ok(())
}
