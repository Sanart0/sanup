use crate::ui::input::inputfieldtype::InputType;
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout, Offset, Rect},
    text::Line,
    widgets::Widget,
};
use toml::Value;

#[derive(Clone)]
pub struct InputField<T: InputType> {
    title: &'static str,
    value: T,
}

impl<T: InputType + Default> InputField<T> {
    pub fn new(label: &'static str) -> Self {
        Self {
            title: label,
            value: T::default(),
        }
    }

    pub fn new_with_value(label: &'static str, value: T) -> Self {
        Self {
            title: label,
            value,
        }
    }

    pub fn value(&self) -> T::Value {
        self.value.value()
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.value.set_focus(focus);
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        self.value.on_key(key);
    }

    pub fn cursor_offset(&self) -> Offset {
        Offset {
            x: (self.title.len() + self.value.to_string().len() + 2) as i32,
            y: 0,
        }
    }
}

impl<T: InputType> Widget for &InputField<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([
            Constraint::Length(self.title.len() as u16 + 2),
            Constraint::Fill(1),
        ]);
        let [label_area, value_area] = layout.areas(area);
        let label = Line::from_iter([self.title, ": "]);
        let value = self.value.to_string();
        label.render(label_area, buf);
        value.render(value_area, buf);
    }
}
