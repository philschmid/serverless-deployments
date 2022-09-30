use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_lambda::{
    model::{FunctionCode, PackageType},
    Client,
};
use std::env;
use tracing::info;

const REGION: &str = "us-east-1";
const AWS_PROFILE: &str = "hf-sm";

async fn get_credentials() -> SdkConfig {
    match env::var("AWS_ACCESS_KEY_ID") {
        Ok(_) => (),
        Err(_) => env::set_var("AWS_PROFILE", AWS_PROFILE),
    }
    let region_provider = RegionProviderChain::default_provider().or_else(REGION);
    aws_config::from_env().region(region_provider).load().await
}

struct LambdaHandler {
    client: Client,
}

impl LambdaHandler {
    async fn new() -> Result<Self> {
        let config = get_credentials().await;
        let client = Client::new(&config);
        Ok(Self { client })
    }

    async fn exists(&self, name: &str) -> Option<aws_sdk_lambda::output::GetFunctionOutput> {
        let exists = self.client.get_function().function_name(name).send().await;
        info!("Function {} exists: {}", name, exists.is_ok());

        match exists {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
    async fn create(
        &self,
        name: &str,
        ecr_uri: &str,
        role: &str,
        memory_size: Option<i32>,
    ) -> Result<()> {
        info!("Creating lambda function {}", name);
        let create = self
            .client
            .create_function()
            .function_name(name)
            .package_type(PackageType::Image)
            .code(FunctionCode::builder().image_uri(ecr_uri).build())
            .role(role)
            .memory_size(memory_size.unwrap_or(2048))
            .send()
            .await?;
        info!("Created lambda function {}", name);
        Ok(())
    }
    async fn remove(&self, name: &str) -> Result<()> {
        info!("Removing lambda function {}", name);
        let remove = self
            .client
            .delete_function()
            .function_name(name)
            .send()
            .await?;
        info!("Removed lambda function {}", name);
        Ok(())
    }
}

pub async fn deploy(name: &str) -> Result<()> {
    let lh = LambdaHandler::new().await?;

    match lh.exists(name).await {
        Some(_) => lh.remove(name).await?,
        None => lh.create(
            name,
            "558105141721.dkr.ecr.us-east-1.amazonaws.com/huggingface-inference-pytorch:1.8.1-cpu",
            "arn:aws:iam::558105141721:role/artilleryio-default-lambda-role",
            Some(2048),
        ).await?,
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
