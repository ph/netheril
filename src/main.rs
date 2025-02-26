use cli::handle_cli;

mod actor;
mod api;
mod app;
mod cli;
mod error;
mod logging;
mod services;
mod version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    handle_cli().await?;
    Ok(())
}
