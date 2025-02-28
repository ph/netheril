use cli::handle_cli;

mod actor;
mod api;
mod app;
mod cli;
mod error;
mod logging;
mod services;
mod version;

pub async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    handle_cli().await?;
    Ok(())
}
