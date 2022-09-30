use anyhow::Result;
use clap::Parser;
use serverless_deploy::deploy;
use std::env;
use tracing::info;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of lambda function
    #[arg(short, long)]
    function_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::INFO)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    env::set_var("RUST_LOG", "INFO");

    tracing_subscriber::fmt::init();
    info!("test");

    let args = Args::parse();
    deploy(args.function_name.as_str()).await?;
    Ok(())
}
