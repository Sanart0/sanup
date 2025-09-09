use ratatui::{
    buffer::Buffer, crossterm::event::{KeyCode, KeyEvent}, layout::{Constraint, Layout, Offset, Rect}, style::Stylize, text::Line, widgets::Widget
};

pub struct StringField {
    lable: &'static str,
    value: String,
}

impl StringField {
    pub fn new(lable: &'static str) -> Self {
        Self {
            lable,
            value: String::new(),
        }
    }

    pub fn fill(mut self, value: String) -> Self {
        self.value.clone_from(&value);
        self
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.value.push(c),
            KeyCode::Backspace => {
                self.value.pop();
            }
            _ => {}
        }
    }

    pub fn cursor_offset(&self) -> Offset {
        Offset {
            x: (self.lable.len() + self.value.len() + 2) as i32,
            y: 0,
        }
    }
}

impl Widget for StringField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([
            Constraint::Length(self.lable.len() as u16 + 2),
            Constraint::Fill(1),
        ]);
        let [label_area, value_area] = layout.areas(area);
        let label = Line::from_iter([self.lable, ": "]).bold();
        label.render(label_area, buf);
        self.value.render(value_area, buf);
    }
}
