use crate::ui::input::inputfieldtype::InputType;
use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout, Offset, Rect},
    text::Line,
    widgets::Widget,
};

#[derive(Clone)]
pub enum InputFieldKind {
    Bool,
    Integer,
    Float,
    String,
    Enum,
}

#[derive(Clone)]
pub struct InputField<T: InputType> {
    title: String,
    kind: InputFieldKind,
    value: T,
}

impl<T: InputType> InputField<T> {
    pub fn new(label: &str) -> Self {
        Self {
            title: label.to_string(),
            kind: T::kind(),
            value: T::default(),
        }
    }

    pub fn new_with_value(label: &str, value: T) -> Self {
        Self {
            title: label.to_string(),
            kind: T::kind(),
            value,
        }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn field(&self) -> &T {
        &self.value
    }

    pub fn field_mut(&mut self) -> &mut T {
        &mut self.value
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
            x: if let InputFieldKind::Enum = self.kind {
                self.title.len() + 1
            } else {
                self.title.len() + self.value.to_string().len() + 2
            } as i32,
            y: 0,
        }
    }
}

impl<T: InputType + Widget + Clone> Widget for &InputField<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([
            Constraint::Length(self.title.len() as u16 + 2),
            Constraint::Fill(1),
        ]);

        let [label_area, value_area] = layout.areas(area);
        let label = Line::from_iter([self.title.as_str(), ": "]);

        label.render(label_area, buf);
        self.value.clone().render(value_area, buf);
    }
}
