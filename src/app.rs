use tracing::info;

use crate::logging::{Logging, LoggingOptions};

pub struct App {
    logging: Logging,
}

impl App {
    pub fn new() -> Self {
        info!("configuring");
        let logging = Logging::new(LoggingOptions::default());
        App { logging }
    }

    pub fn run(&self) {
        info!("starting");
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
