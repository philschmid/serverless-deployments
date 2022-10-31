mod error;
use crate::error::ServerlessDeployError;
use anyhow::Result;
use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_lambda::{
    model::{Cors, FunctionCode, FunctionUrlAuthType, PackageType},
    output::GetFunctionOutput,
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

    async fn get(&self, function_name: &str) -> Result<GetFunctionOutput> {
        Ok(self
            .client
            .get_function()
            .function_name(function_name)
            .send()
            .await?)
    }

    async fn exists(&self, name: &str) -> Option<bool> {
        match self.get(name).await {
            Ok(_) => {
                info!("Function {} exists", name);
                Some(true)
            }
            Err(_) => None,
        }
    }

    async fn create(
        &self,
        name: &str,
        ecr_uri: &str,
        role: &str,
        memory_size: Option<i32>,
        create_url: Option<bool>,
    ) -> Result<Option<String>> {
        info!("Creating lambda function {}", name);
        self.client
            .create_function()
            .function_name(name)
            .package_type(PackageType::Image)
            .code(FunctionCode::builder().image_uri(ecr_uri).build())
            .role(role)
            .memory_size(memory_size.unwrap_or(2048))
            .send()
            .await
            .map_err(|_| ServerlessDeployError::Creation)?;
        info!("Created lambda function {}", name);
        if create_url.unwrap_or(false) {
            info!("Create Function URL");
            let url_config = self
                .client
                .create_function_url_config()
                .function_name(name)
                .auth_type(FunctionUrlAuthType::None)
                .cors(
                    Cors::builder()
                        .allow_origins("*")
                        .allow_methods("*")
                        .build(),
                )
                .send()
                .await?;

            info!("{:?}", url_config);

            let url = match url_config.function_url() {
                Some(url) => {
                    info!("assigned {} to function {}", url, name);
                    url
                }
                None => {
                    info!("No URL assigned to function {}", name);
                    return Ok(None);
                }
            };
            return Ok(Some(url.to_owned()));
        }
        Ok(None)
    }

    async fn remove(&self, name: &str) -> Result<()> {
        info!("Removing lambda function {}", name);
        self.client
            .delete_function()
            .function_name(name)
            .send()
            .await?;
        info!("Removed lambda function {}", name);
        Ok(())
    }
}

pub async fn deploy(name: &str) -> Result<(), ServerlessDeployError> {
    let lh = LambdaHandler::new().await?;

    // Check if function exists returns Err if function already exists
    if (lh.exists(name).await).is_some() {
        return Err(ServerlessDeployError::AlreadyExists(name.to_string()));
    }
    // Create function#
    lh.create(
        name,
        "558105141721.dkr.ecr.us-east-1.amazonaws.com/huggingface-inference-pytorch:1.8.1-cpu",
        "arn:aws:iam::558105141721:role/artilleryio-default-lambda-role",
        Some(2048),
        Some(true),
    )
    .await?;
    //

    Ok(())
}

pub async fn remove(name: &str) -> Result<()> {
    let lh = LambdaHandler::new().await?;
    lh.remove(name).await?;
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
