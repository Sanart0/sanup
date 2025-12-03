use crate::ui::input::enumvariants::EnumVariants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CompressionLevel {
    Default,
    Best,
}

impl EnumVariants for CompressionLevel {
    fn default(&self) -> Box<dyn EnumVariants> {
        Box::new(CompressionLevel::Default)
    }

    fn longest(&self) -> String {
        CompressionLevel::Default.to_string()
    }

    fn variants(&self) -> Vec<String> {
        ["Default", "Best"].iter().map(|s| s.to_string()).collect()
    }

    fn from_string(&self, s: String) -> Box<dyn EnumVariants> {
        Box::new(match s.as_str() {
            "None" => CompressionLevel::Default,
            "Zip" => CompressionLevel::Best,
            _ => CompressionLevel::Default,
        })
    }

    fn clone_box(&self) -> Box<dyn EnumVariants> {
        Box::new(self.clone())
    }
}

impl ToString for CompressionLevel {
    fn to_string(&self) -> String {
        match self {
            CompressionLevel::Default => "Default",
            CompressionLevel::Best => "Best",
        }
        .to_string()
    }
}
