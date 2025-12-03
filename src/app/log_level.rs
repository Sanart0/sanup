use crate::ui::input::enumvariants::EnumVariants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl EnumVariants for LogLevel {
    fn default(&self) -> Box<dyn EnumVariants> {
        Box::new(LogLevel::Info)
    }

    fn longest(&self) -> String {
        LogLevel::Error.to_string()
    }

    fn variants(&self) -> Vec<String> {
        [""].iter().map(|s| s.to_string()).collect()
    }

    fn from_string(&self, s: String) -> Box<dyn EnumVariants> {
        Box::new(match s.as_str() {
            "Error" => LogLevel::Error,
            "Warn" => LogLevel::Warn,
            "Info" => LogLevel::Info,
            "Debug" => LogLevel::Debug,
            "Trace" => LogLevel::Trace,
            _ => LogLevel::Info,
        })
    }

    fn clone_box(&self) -> Box<dyn EnumVariants> {
        Box::new(self.clone())
    }
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Error => "Error",
            LogLevel::Warn => "Warn",
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
            LogLevel::Trace => "Trace",
        }
        .to_string()
    }
}
