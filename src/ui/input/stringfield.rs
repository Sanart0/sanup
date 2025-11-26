use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::ui::input::inputfieldtype::InputType;

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

#[allow(clippy::to_string_trait_impl)]
impl ToString for StringField {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl InputType for StringField {
    type Value = String;

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
