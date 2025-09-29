use crate::error::SanupResult;
use chrono::{Local, Utc};
use fern::{
    Dispatch,
    colors::{Color, ColoredLevelConfig},
};
use log::{LevelFilter, Log};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

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

    fn change(
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

    pub fn init<P: AsRef<Path>>(self: Arc<Self>, path: P) -> SanupResult<()> {
        let mut file_path = path.as_ref().to_path_buf();
        file_path.push(format!(
            "log_{}.log",
            Utc::now().date_naive().format("%Y-%m-%d")
        ));

        self.change(
            Dispatch::new()
                .format(|out, msg, record| {
                    out.finish(format_args!(
                        "[{}] [{}] {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        SanupLogger::full_colors().color(record.level()),
                        msg
                    ));
                })
                .chain(fern::log_file(file_path)?)
                .into_log(),
        )?;

        Ok(())
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
        if let Ok(logger) = self.logger.lock() {
            logger.log(record)
        }
    }

    fn flush(&self) {
        if let Ok(logger) = self.logger.lock() {
            logger.flush()
        }
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
