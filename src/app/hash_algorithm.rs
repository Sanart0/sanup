use crate::ui::input::enumvariants::EnumVariants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum HashAlgorithm {
    Sha256,
}

impl EnumVariants for HashAlgorithm {
    fn default(&self) -> Box<dyn EnumVariants> {
        Box::new(HashAlgorithm::Sha256)
    }

    fn longest(&self) -> String {
        HashAlgorithm::Sha256.to_string()
    }

    fn variants(&self) -> Vec<String> {
        ["Sha256"].iter().map(|s| s.to_string()).collect()
    }

    fn from_string(&self, s: String) -> Box<dyn EnumVariants> {
        Box::new(match s.as_str() {
            "Sha256" => HashAlgorithm::Sha256,
            _ => HashAlgorithm::Sha256,
        })
    }

    fn clone_box(&self) -> Box<dyn EnumVariants> {
        Box::new(self.clone())
    }
}

impl ToString for HashAlgorithm {
    fn to_string(&self) -> String {
        match self {
            HashAlgorithm::Sha256 => "Sha256",
        }
        .to_string()
    }
}
