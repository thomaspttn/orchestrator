use aws_config::meta::region::RegionProviderChain;
use aws_types::SdkConfig;
use std::env;

/// Holds application-wide configuration
#[derive(Debug)]
pub struct AppConfig {
    pub aws_region: Option<String>,
    pub aws_profile: Option<String>,
    pub debug_mode: bool,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            aws_region: env::var("AWS_REGION").ok(),
            aws_profile: env::var("AWS_PROFILE").ok(),
            debug_mode: env::var("DEBUG_MODE")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
        }
    }

    pub async fn aws_sdk_config(&self) -> SdkConfig {
        // Set up RegionProviderChain: Environment > AWS Config Defaults
        let region_provider = if let Some(region) = &self.aws_region {
            RegionProviderChain::first_try(Some(aws_sdk_ec2::Region::new(region.clone())))
        } else {
            RegionProviderChain::default_provider()
        };

        // Initialize AWS SDK config loader
        let mut config_loader = aws_config::from_env().region(region_provider);

        // Apply AWS_PROFILE if specified
        if let Some(profile) = &self.aws_profile {
            config_loader = config_loader.profile_name(profile);
        }

        config_loader.load().await
    }
}
