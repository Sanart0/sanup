use crate::error::SanupResult;
use fern::colors::{Color, ColoredLevelConfig};
use log::{LevelFilter, Log};
use std::sync::{Arc, Mutex};

pub struct SanupLogger {
    logger: Mutex<Box<dyn Log>>,
}

#[allow(clippy::should_implement_trait)]
impl SanupLogger {
    pub fn default() -> Arc<Self> {
        let logger = Arc::new(SanupLogger {
            logger: Mutex::new(Box::new(EmptyLogger)),
        });

        if log::set_boxed_logger(Box::new(logger.clone())).is_ok() {};

        log::set_max_level(LevelFilter::Off);

        logger
    }

    pub fn is_logger_set() -> Option<Arc<Self>> {
        let logger = Self::default();

        if log::set_boxed_logger(Box::new(logger.clone())).is_ok() {
            Some(logger)
        } else {
            None
        }
    }

    pub fn change(
        self: Arc<Self>,
        (level, new_logger): (LevelFilter, Box<dyn Log>),
    ) -> SanupResult<()> {
        *self.logger.lock()? = new_logger;
        log::set_max_level(level);

        Ok(())
    }

    pub fn full_colors() -> ColoredLevelConfig {
        ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::Blue)
            .debug(Color::Green)
            .trace(Color::Magenta)
    }
}

impl Log for SanupLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        match self.logger.lock() {
            Ok(logger) => logger.enabled(metadata),
            Err(_) => false,
        }
    }

    fn log(&self, record: &log::Record) {
        if let Ok(logger) = self.logger.lock() { logger.log(record) }
    }

    fn flush(&self) {
        if let Ok(logger) = self.logger.lock() { logger.flush() }
    }
}

pub struct EmptyLogger;

impl Log for EmptyLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        false
    }
    fn log(&self, _record: &log::Record) {}
    fn flush(&self) {}
}
