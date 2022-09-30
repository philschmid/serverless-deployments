use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_lambda::{Client, Error};
use std::env;

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
    async fn new() -> Result<Self, Error> {
        let config = get_credentials().await;
        let client = Client::new(&config);
        Ok(Self { client })
    }

    async fn exists(&self, name: &str) -> Option<aws_sdk_lambda::output::GetFunctionOutput> {
        let exists = self.client.get_function().function_name(name).send().await;

        match exists {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
}

pub async fn deploy(name: &str) -> Result<(), Error> {
    let lh = LambdaHandler::new().await?;

    match lh.exists(name).await {
        Some(_) => println!("{} exists", name),
        None => println!("{} does not exist", name),
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
