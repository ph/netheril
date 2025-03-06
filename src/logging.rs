use tracing::{level_filters::LevelFilter, trace, Level};
use tracing_subscriber::{
    fmt,
    prelude::*,
    reload::{self, Handle},
    Registry,
};

use crate::error::NetherilErr;

#[allow(dead_code)]
pub struct Logging {
    reload_handle: Handle<LevelFilter, Registry>,
    logging_options: LoggingOptions,
}

impl Logging {
    pub fn new(options: LoggingOptions) -> Self {
        let level: LevelFilter = options.level.into();
        let (filter, reload_handle) = reload::Layer::new(level);

        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::Layer::default())
            .init();

        Logging {
            reload_handle,
            logging_options: options,
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, options: LoggingOptions) -> Result<(), NetherilErr> {
        trace!("update: {:?}", options);

        self.reload_handle
            .modify(|filter| {
                let f: LevelFilter = options.level.into();
                *filter = f
            })
            .map_err(|e| NetherilErr::Logging(e.to_string()))?;

        self.logging_options = options;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LoggingOptions {
    level: Level,
}

impl Default for LoggingOptions {
    fn default() -> Self {
        LoggingOptions {
            level: Level::DEBUG,
        }
    }
}
