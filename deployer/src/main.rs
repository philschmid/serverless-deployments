use anyhow::Result;
use clap::Parser;
use serverless_deploy::{deploy, remove};
use std::env;
use tracing::info;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of lambda function
    #[arg(short, long, default_value = "hello")]
    function_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "INFO");

    tracing_subscriber::fmt::init();
    info!("test");

    let args = Args::parse();
    remove(&args.function_name).await?;
    deploy(args.function_name.as_str()).await?;
    Ok(())
}
