use crate::cli::cli;

mod actor;
mod app;
mod cli;
mod error;
mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli()
}
