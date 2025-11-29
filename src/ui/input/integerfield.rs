use crate::ui::input::{inputfield::InputFieldKind, inputfieldtype::InputType};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::Widget,
};

#[derive(Clone, Default)]
pub struct IntegerField {
    focus: bool,
    value: i64,
}

impl IntegerField {
    pub fn parse_input(&mut self, c: char) {
        if c.is_ascii_digit() {
            self.value = self
                .value
                .saturating_mul(10)
                .saturating_add(c.to_digit(10).unwrap() as i64);
        } else if c == '-' {
            self.value = self.value.saturating_mul(-1);
        }
    }

    pub fn remove_last(&mut self) {
        self.value = self.value.saturating_div(10);
    }

    pub fn increment(&mut self) {
        self.value = self.value.saturating_add(1);
    }

    pub fn decrement(&mut self) {
        self.value = self.value.saturating_sub(1);
    }
}

impl From<i64> for IntegerField {
    fn from(value: i64) -> Self {
        IntegerField {
            focus: false,
            value,
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for IntegerField {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl InputType for IntegerField {
    type Value = i64;

    fn kind() -> InputFieldKind {
        InputFieldKind::Integer
    }

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

impl Widget for IntegerField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let value = self.value.to_string();

        value.render(area, buf);
    }
}
