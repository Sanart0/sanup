use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::ui::input::inputfieldtype::InputType;

#[derive(Clone, Default)]
pub struct FloatField {
    focus: bool,
    value: f64,
    input: String,
    decimal_entered: bool,
}

impl FloatField {
    pub fn parse_input(&mut self, c: char) {
        if c.is_ascii_digit() {
            self.input.push(c);
        } else if c == '-' {
            if self.input.chars().nth(0) == Some('-') {
                self.input.remove(0);
            } else {
                self.input.insert(0, c);
            }
        } else if c == '.' && !self.decimal_entered {
            self.input.push(c);
            self.decimal_entered = true;
        }
        self.value = self.input.parse().unwrap_or(self.value);
    }

    pub fn remove_last(&mut self) {
        if let Some(last) = self.input.pop() {
            if last == '.' {
                self.decimal_entered = false;
            }
            self.value = self.input.parse().unwrap_or(0.0);
        }
    }

    pub fn increment(&mut self) {
        self.value += 1.0;
        self.input = self.value.to_string();
    }

    pub fn decrement(&mut self) {
        self.value -= 1.0;
        self.input = self.value.to_string();
    }
}

impl From<f64> for FloatField {
    fn from(value: f64) -> Self {
        FloatField {
            focus: false,
            value,
            input: value.to_string(),
            decimal_entered: true,
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for FloatField {
    fn to_string(&self) -> String {
        self.input.clone()
    }
}

impl InputType for FloatField {
    type Value = f64;

    fn value(&self) -> Self::Value {
        self.value
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('k') => self.increment(),
            KeyCode::Char('j') => self.decrement(),
            KeyCode::Char(c) => self.parse_input(c),
            KeyCode::Backspace => self.remove_last(),
            _ => {}
        }
    }
}
