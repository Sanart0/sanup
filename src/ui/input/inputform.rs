use crate::ui::{
    centered_rect,
    input::{
        enumfield::EnumFieldState,
        field::{Field, Fields},
        value::{Value, Values},
    },
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Margin, Rect},
    widgets::{Block, Borders, Widget},
};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputFormState {
    Hidden,
    Active,
    Submitted,
    Cancelled,
}

pub struct InputForm {
    state: InputFormState,
    title: &'static str,
    focus_idx: usize,
    fields: Fields,
    field_areas: Rc<[Rect]>,
    inner_area: Rect,
}

impl InputForm {
    pub fn new(label: &'static str, fields: Fields) -> Self {
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

    pub fn values(&self) -> Values {
        self.fields.values()
    }

    fn next_focus(&mut self) {
        self.focus_idx += 1;
        self.focus_idx %= self.fields.len();

        for (i, field) in self.fields.iter_mut().enumerate() {
            match field {
                Field::Bool(f) => f.set_focus(i == self.focus_idx),
                Field::Integer(f) => f.set_focus(i == self.focus_idx),
                Field::Float(f) => f.set_focus(i == self.focus_idx),
                Field::String(f) => f.set_focus(i == self.focus_idx),
                Field::Enum(f) => f.set_focus(i == self.focus_idx),
            }
        }
    }

    fn get_current_enum_field_state(&self) -> EnumFieldState {
        if let Some(Field::Enum(input_field_enum)) = self.fields.get(self.focus_idx) {
            input_field_enum.field().state()
        } else {
            EnumFieldState::Hidden
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        match self.get_current_enum_field_state() {
            EnumFieldState::Hidden => match key.code {
                KeyCode::Esc => {
                    self.state = InputFormState::Cancelled;
                }
                KeyCode::Enter => {
                    self.state = InputFormState::Submitted;
                }
                KeyCode::Tab => {
                    self.next_focus();
                }
                _ => {
                    if let Some(field) = self.fields.get_mut(self.focus_idx) {
                        match field {
                            Field::Bool(f) => f.on_key(key),
                            Field::Integer(f) => f.on_key(key),
                            Field::Float(f) => f.on_key(key),
                            Field::String(f) => f.on_key(key),
                            Field::Enum(f) => f.on_key(key),
                        }
                    }
                }
            },
            EnumFieldState::Active => {
                if let Some(field) = self.fields.get_mut(self.focus_idx) {
                    if let Field::Enum(f) = field {
                        f.on_key(key);
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
                Field::Bool(f) => focused_area.offset(f.cursor_offset()),
                Field::Integer(f) => focused_area.offset(f.cursor_offset()),
                Field::Float(f) => focused_area.offset(f.cursor_offset()),
                Field::String(f) => focused_area.offset(f.cursor_offset()),
                Field::Enum(f) => focused_area.offset(f.cursor_offset()),
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
                Field::Bool(f) => f.render(area, buf),
                Field::Integer(f) => f.render(area, buf),
                Field::Float(f) => f.render(area, buf),
                Field::String(f) => f.render(area, buf),
                Field::Enum(f) => f.render(area, buf),
            }
        }
    }
}

impl Default for InputForm {
    fn default() -> Self {
        Self {
            state: InputFormState::Hidden,
            title: "",
            focus_idx: 0,
            fields: Fields::default(),
            field_areas: Rc::new([]),
            inner_area: Rect::ZERO,
        }
    }
}
