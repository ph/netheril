use crate::cli::cli;

mod app;
mod cli;
mod error;
mod logging;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli()
}
