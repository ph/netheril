use cli::handle_cli;

mod actor;
mod cli;
mod logging;
pub mod actors;
pub mod api;
pub mod app;
pub mod models;
pub mod error;
pub mod services;
pub mod version;

pub async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    handle_cli().await?;
    Ok(())
}
