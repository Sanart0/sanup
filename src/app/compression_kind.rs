use crate::ui::input::enumvariants::EnumVariants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CompressionKind {
    None,
    Zip,
}

impl EnumVariants for CompressionKind {
    fn default(&self) -> Box<dyn EnumVariants> {
        Box::new(CompressionKind::None)
    }

    fn longest(&self) -> String {
        CompressionKind::None.to_string()
    }

    fn variants(&self) -> Vec<String> {
        ["None", "Zip"].iter().map(|s| s.to_string()).collect()
    }

    fn from_string(&self, s: String) -> Box<dyn EnumVariants> {
        Box::new(match s.as_str() {
            "None" => CompressionKind::None,
            "Zip" => CompressionKind::Zip,
            _ => CompressionKind::None,
        })
    }

    fn clone_box(&self) -> Box<dyn EnumVariants> {
        Box::new(self.clone())
    }
}

impl ToString for CompressionKind {
    fn to_string(&self) -> String {
        match self {
            CompressionKind::None => "None",
            CompressionKind::Zip => "Zip",
        }
        .to_string()
    }
}
