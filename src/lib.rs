use cli::handle_cli;

mod actor;
mod cli;
mod logging;
mod operation;
pub mod api;
pub mod app;
pub mod domains;
pub mod error;
pub mod services;
pub mod version;

pub async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    handle_cli().await?;
    Ok(())
}
