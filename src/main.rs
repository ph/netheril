use cli::handle_cli;

mod actor;
mod app;
mod cli;
mod error;
mod logging;
mod version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    handle_cli().await?;
    Ok(())
}
