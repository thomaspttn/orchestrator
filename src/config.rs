use aws_sdk_ec2::config::Builder as Ec2ConfigBuilder;
use aws_sdk_ec2::config::Region;
use aws_sdk_ec2::Client;
use std::env;

/// Holds application-wide configuration
#[derive(Debug)]
pub struct AppConfig {
    pub aws_region: Option<String>,
    pub aws_profile: Option<String>,
    pub debug_mode: bool,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            aws_region: env::var("AWS_REGION").ok(),
            aws_profile: env::var("AWS_PROFILE").ok(),
            debug_mode: env::var("DEBUG_MODE")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
        }
    }

    /// Initialize the AWS SDK EC2 client configuration
    pub async fn aws_ec2_client(&self) -> Client {
        // Set the AWS region, falling back to `us-east-1` if none is provided
        let region = Region::new(
            self.aws_region
                .clone()
                .unwrap_or_else(|| "us-east-1".to_string()),
        );
        let region_config = Region::new(region);

        // Set the AWS_PROFILE if provided
        if let Some(profile) = &self.aws_profile {
            env::set_var("AWS_PROFILE", profile);
        }

        // Build the EC2 configuration
        let ec2_config = Ec2ConfigBuilder::new().region(region_config).build();

        // Return an EC2 client
        Client::from_conf(ec2_config)
    }
}
