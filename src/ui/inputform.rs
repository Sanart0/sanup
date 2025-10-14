use std::rc::Rc;

use log::info;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Margin, Rect},
    widgets::{Block, Borders, Widget},
};

use crate::ui::{
    inputfield::InputField,
    inputfieldtype::{FloatField, IntegerField, StringField},
};

#[derive(Clone)]
pub struct InputForm {
    state: InputFormState,
    title: &'static str,
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

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i128),
    Float(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputFormState {
    Hidden,
    Active,
    Submitted,
    Cancelled,
}

impl InputForm {
    pub fn empty() -> Self {
        Self {
            state: InputFormState::Hidden,
            title: "",
            focus_idx: 0,
            fields: Vec::new(),
            field_areas: Rc::new([]),
            inner_area: Rect::ZERO,
        }
    }

    pub fn new(label: &'static str, fields: Vec<Field>) -> Self {
        Self {
            state: InputFormState::Active,
            title: label,
            focus_idx: 0,
            fields,
            field_areas: Rc::new([]),
            inner_area: Rect::ZERO,
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == InputFormState::Active
    }

    pub fn is_submitted(&self) -> bool {
        self.state == InputFormState::Submitted
    }

    pub fn is_cancelled(&self) -> bool {
        self.state == InputFormState::Cancelled
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

    fn next_focus(&mut self) {
        self.focus_idx += 1;
        self.focus_idx %= self.fields.len();

        for (i, field) in self.fields.iter_mut().enumerate() {
            match field {
                Field::String(f) => f.set_focus(i == self.focus_idx),
                Field::Integer(f) => f.set_focus(i == self.focus_idx),
                Field::Float(f) => f.set_focus(i == self.focus_idx),
            }
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        info!("PRESSED: {:?}", key);
        info!("STATE: {:?}", self.state);
        match key.code {
            KeyCode::Tab => {
                self.next_focus();
            }
            KeyCode::Enter => {
                self.state = InputFormState::Submitted;
            }
            KeyCode::Esc => {
                self.state = InputFormState::Cancelled;
            }
            _ => {
                if let Some(field) = self.fields.get_mut(self.focus_idx) {
                    match field {
                        Field::String(f) => f.on_key(key),
                        Field::Integer(f) => f.on_key(key),
                        Field::Float(f) => f.on_key(key),
                    }
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

        if let Some(field) = focused_field {
            match field {
                Field::String(f) => focused_area.offset(f.cursor_offset()),
                Field::Integer(f) => focused_area.offset(f.cursor_offset()),
                Field::Float(f) => focused_area.offset(f.cursor_offset()),
            }
        } else {
            focused_area
        }
    }
}

impl Widget for &mut InputForm {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let height = self.fields.len() as u16 + 3;
        let popup_area = centered_rect(area, 50, height);

        let block = Block::default().title(self.title).borders(Borders::ALL);
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
