use anyhow::Result;
use serverless_deploy::deploy;

use clap::Parser;
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
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    deploy(args.function_name.as_str()).await?;
    Ok(())
}
