use crate::ui::input::enumvariants::EnumVariants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Theme {
    Dark,
    Kanagawa,
    Nord,
    Dracula,
    TokyoNight,
}

impl EnumVariants for Theme {
    fn default(&self) -> Box<dyn EnumVariants> {
        Box::new(Theme::Dark)
    }

    fn longest(&self) -> String {
        Theme::TokyoNight.to_string()
    }

    fn variants(&self) -> Vec<String> {
        ["Dark", "Kanagawa", "Nord", "Dracula", "TokyoNight"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn from_string(&self, s: String) -> Box<dyn EnumVariants> {
        Box::new(match s.as_str() {
            "Dark" => Theme::Dark,
            "Kanagawa" => Theme::Kanagawa,
            "Nord" => Theme::Nord,
            "Dracula" => Theme::Dracula,
            "TokyoNight" => Theme::TokyoNight,
            _ => Theme::Dark,
        })
    }

    fn clone_box(&self) -> Box<dyn EnumVariants> {
        Box::new(self.clone())
    }
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Theme::Dark => "Dark",
            Theme::Kanagawa => "Kanagawa",
            Theme::Nord => "Nord",
            Theme::Dracula => "Dracula",
            Theme::TokyoNight => "TokyoNight",
        }
        .to_string()
    }
}
