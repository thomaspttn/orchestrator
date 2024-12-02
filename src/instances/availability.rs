use aws_sdk_ec2::error::SdkError;
use aws_sdk_ec2::operation::describe_instance_type_offerings::DescribeInstanceTypeOfferingsError;
use aws_sdk_ec2::operation::describe_spot_price_history::DescribeSpotPriceHistoryError;
use aws_sdk_ec2::types::Filter;
use aws_sdk_ec2::types::LocationType;
use aws_sdk_ec2::Client;

pub struct AvailabilityChecker {
    ec2_client: Client,
}

impl AvailabilityChecker {
    pub fn new(ec2_client: Client) -> Self {
        Self { ec2_client }
    }

    pub async fn check_availability(
        &self,
        instance_type: &str,
    ) -> Result<(), SdkError<DescribeInstanceTypeOfferingsError>> {
        log::info!("Checking availability for instance type: {}", instance_type);

        let result = self
            .ec2_client
            .describe_instance_type_offerings()
            .filters(
                Filter::builder()
                    .name("instance-type")
                    .values(instance_type)
                    .build(),
            )
            .location_type(LocationType::AvailabilityZone)
            .send()
            .await?;

        for offering in result.instance_type_offerings().unwrap_or_default() {
            log::info!(
                "Found offering: {:?} in zone: {:?}",
                offering.instance_type(),
                offering.location()
            );
        }

        Ok(())
    }

    pub async fn check_spot_prices(
        &self,
        instance_type: &str,
        region: &str,
    ) -> Result<(), Result<(), SdkError<DescribeSpotPriceHistoryError>>> {
        log::info!(
            "Checking spot prices for instance type: {} in region: {}",
            instance_type,
            region
        );

        let result = self
            .ec2_client
            .describe_spot_price_history()
            //.instance_types(instance_type)
            //.instance_types(instance_type)
            //.product_descriptions("Linux/UNIX")
            //.start_time("2021-01-01T00:00:00Z")
            //.end_time("2021-01-02T00:00:00Z")
            //.region(region)
            .send()
            .await?;

        Ok(())
    }
}
