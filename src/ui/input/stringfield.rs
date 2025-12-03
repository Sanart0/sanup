use std::path::PathBuf;

use crate::ui::input::{inputfield::InputFieldKind, inputfieldtype::InputType};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::Widget,
};

#[derive(Clone, Default)]
pub struct StringField {
    focus: bool,
    value: String,
}

impl StringField {
    pub fn parse_input(&mut self, c: char) {
        if c.is_ascii() && !c.is_control() {
            self.value.push(c);
        }
    }

    pub fn remove_last(&mut self) {
        self.value.pop();
    }
}

impl From<String> for StringField {
    fn from(value: String) -> Self {
        StringField {
            focus: false,
            value,
        }
    }
}

impl From<PathBuf> for StringField {
    fn from(value: PathBuf) -> Self {
        if let Some(str) = value.to_str() {
            StringField {
                focus: false,
                value: str.to_string(),
            }
        } else {
            StringField {
                focus: false,
                value: String::from(""),
            }
        }
    }
}

impl From<Option<PathBuf>> for StringField {
    fn from(value: Option<PathBuf>) -> Self {
        if let Some(value) = value {
            if let Some(str) = value.to_str() {
                return StringField {
                    focus: false,
                    value: str.to_string(),
                };
            }
        }
        StringField {
            focus: false,
            value: String::from(""),
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for StringField {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl InputType for StringField {
    type Value = String;

    fn kind() -> InputFieldKind {
        InputFieldKind::String
    }

    fn value(&self) -> Self::Value {
        self.value.clone()
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.parse_input(c),
            KeyCode::Backspace => self.remove_last(),
            _ => {}
        }
    }
}

impl Widget for StringField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.value.render(area, buf);
    }
}
