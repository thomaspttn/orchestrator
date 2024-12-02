mod config;
mod instances;

use aws_sdk_ec2::Client as Ec2Client;
use config::AppConfig;
use instances::availability::AvailabilityChecker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load application configuration
    let app_config = AppConfig::from_env();

    // Set up logging
    if app_config.debug_mode {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }

    log::info!(
        "Starting spot-orchestrator with configuration: {:?}",
        app_config
    );

    // Initialize AWS SDK config
    let aws_config = app_config.aws_sdk_config().await;
    let ec2_client = Ec2Client::new(&aws_config);

    // Test AWS connection by listing regions
    let regions = ec2_client.describe_regions().send().await?;
    log::info!("Available regions: {:?}", regions.regions());

    let checker = AvailabilityChecker::new(ec2_client);

    // Check availability for a specific instance type
    checker
        .check_availability("t3.micro")
        .await
        .expect("Failed to check instance availability");

    Ok(())
}
