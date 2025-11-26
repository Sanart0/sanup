use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::ui::input::inputfieldtype::InputType;

#[derive(Clone, Default)]
pub struct BoolField {
    focus: bool,
    value: bool,
}

impl BoolField {
    pub fn toggle(&mut self) {
        self.value = !self.value;
    }
}

impl From<bool> for BoolField {
    fn from(value: bool) -> Self {
        BoolField {
            focus: false,
            value,
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for BoolField {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl InputType for BoolField {
    type Value = bool;

    fn value(&self) -> Self::Value {
        self.value
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('k') | KeyCode::Char('j') => self.toggle(),
            _ => {}
        }
    }
}
