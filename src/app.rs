use tracing::info;

use crate::{
    error::NetherilErr,
    logging::{Logging, LoggingOptions},
};

pub struct App {
    #[allow(dead_code)]
    logging: Logging,
}

impl App {
    pub fn new() -> Self {
        info!("configuring");
        let logging = Logging::new(LoggingOptions::default());
        App { logging }
    }

    pub fn run(&self) -> Result<(), Box<NetherilErr>> {
        info!("starting");
        println!("Run!");
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
