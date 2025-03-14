use cli::handle_cli;

mod actor;
pub mod api;
pub mod app;
mod cli;
pub mod domains;
pub mod error;
mod logging;
mod operation;
pub mod services;
pub mod version;

pub async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    handle_cli().await?;
    Ok(())
}
