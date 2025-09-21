use std::rc::Rc;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Margin, Rect}, widgets::{Block, Borders, Widget},
};

use crate::ui::{
    inputfield::InputField,
    inputfieldtype::{FloatField, IntegerField, StringField},
};

#[derive(Clone)]
pub struct InputForm {
    state: FormState,
    focus_idx: usize,
    fields: Vec<Field>,
    field_areas: Rc<[Rect]>,
    inner_area: Rect,
}

#[derive(Clone)]
pub enum Field {
    String(InputField<StringField>),
    Integer(InputField<IntegerField>),
    Float(InputField<FloatField>),
}

#[derive(Clone)]
pub enum Value {
    String(String),
    Integer(i128),
    Float(f64),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FormState {
    Active,
    Submitted,
    Cancelled,
}

impl InputForm {
    pub fn new(fields: Vec<Field>) -> Self {
        let mut form = Self {
            state: FormState::Active,
            focus_idx: 0,
            fields,
            field_areas: Rc::new([]),
            inner_area: Rect::ZERO,
        };
        form.update_focus();
        form
    }

    pub fn is_active(&self) -> bool {
        self.state == FormState::Active
    }

    pub fn is_submitted(&self) -> bool {
        self.state == FormState::Submitted
    }

    pub fn is_cancelled(&self) -> bool {
        self.state == FormState::Cancelled
    }

    pub fn values(&self) -> Vec<Value> {
        self.fields
            .iter()
            .map(|field| match field {
                Field::String(f) => Value::String(f.value()),
                Field::Integer(f) => Value::Integer(f.value()),
                Field::Float(f) => Value::Float(f.value()),
            })
            .collect()
    }

    fn update_focus(&mut self) {
        for (i, field) in self.fields.iter_mut().enumerate() {
            match field {
                Field::String(f) => f.set_focus(i == self.focus_idx),
                Field::Integer(f) => f.set_focus(i == self.focus_idx),
                Field::Float(f) => f.set_focus(i == self.focus_idx),
            }
        }
    }

    pub fn on_key(&mut self, event: KeyEvent) -> bool {
        if !self.is_active() {
            return false;
        }

        match event.code {
            KeyCode::Tab => {
                self.focus_idx += 1;
                self.focus_idx %= self.fields.len();

                self.update_focus();
                true
            }
            KeyCode::Enter => {
                self.state = FormState::Submitted;
                true
            }
            KeyCode::Esc => {
                self.state = FormState::Cancelled;
                true
            }
            _ => {
                if let Some(field) = self.fields.get_mut(self.focus_idx) {
                    match field {
                        Field::String(f) => f.on_key(event),
                        Field::Integer(f) => f.on_key(event),
                        Field::Float(f) => f.on_key(event),
                    }
                } else {
                    false
                }
            }
        }
    }

    pub fn cursor_position(&self) -> Rect {
        let focused_field = self.fields.get(self.focus_idx);

        let focused_area = self
            .field_areas
            .get(self.focus_idx)
            .cloned()
            .unwrap_or(self.inner_area);

        let cursor_position = if let Some(field) = focused_field {
            match field {
                Field::String(f) => focused_area.offset(f.cursor_offset()),
                Field::Integer(f) => focused_area.offset(f.cursor_offset()),
                Field::Float(f) => focused_area.offset(f.cursor_offset()),
            }
        } else {
            focused_area
        };

        return cursor_position;
    }
}

impl Widget for &mut InputForm {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let height = self.fields.len() as u16 + 3;
        let popup_area = centered_rect(area, 50, height);

        let block = Block::default()
            .title("Input Form")
            .borders(Borders::ALL);
        block.render(popup_area, buf);

        self.inner_area = popup_area.inner(Margin::new(1, 1));
        let constraints: Vec<Constraint> = (0..self.fields.len())
            .map(|_| Constraint::Length(1))
            .collect();
        let layout = Layout::vertical(constraints);
        self.field_areas = layout.split(self.inner_area);

        for (i, field) in self.fields.iter().enumerate() {
            let area = self.field_areas[i];
            match field {
                Field::String(f) => f.render(area, buf),
                Field::Integer(f) => f.render(area, buf),
                Field::Float(f) => f.render(area, buf),
            }
        }
    }
}

fn centered_rect(parent: Rect, percent_x: u16, height: u16) -> Rect {
    let width = parent.width * percent_x / 100;
    Rect {
        x: parent.x + (parent.width.saturating_sub(width)) / 2,
        y: parent.y + (parent.height.saturating_sub(height)) / 2,
        width,
        height,
    }
}
