use crate::ui::input::{
    field::{Field, Fields},
    value::Values,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputListState {
    None,
    Active,
    Changed,
    SaveOrReset,
}

pub struct InputList {
    title: Option<String>,
    state: InputListState,
    fields: Fields,
    focus_idx: usize,
    show_border: bool,
}

impl InputList {
    pub fn new(title: Option<&str>, show_border: bool, fields: Fields) -> Self {
        Self {
            title: title.map(|t| t.to_string()),
            state: InputListState::None,
            fields,
            focus_idx: 0,
            show_border,
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == InputListState::Active
    }

    pub fn is_changed(&self) -> bool {
        self.state == InputListState::Changed
    }

    pub fn is_save_or_reset(&self) -> bool {
        self.state == InputListState::SaveOrReset
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

    fn create_list_items(&self) -> Vec<ListItem> {
        self.fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let (label, value) = match field {
                    Field::Bool(f) => (f.title(), f.value().to_string()),
                    Field::Integer(f) => (f.title(), f.value().to_string()),
                    Field::Float(f) => (f.title(), f.value().to_string()),
                    Field::String(f) => (f.title(), f.value().clone()),
                    Field::Enum(f) => (f.title(), f.field().to_string()),
                };

                let line = Line::from(format!("{}: {}", label, value));
                let mut item = ListItem::new(line);

                if idx == self.focus_idx {
                    item = item.style(
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    );
                }

                item
            })
            .collect()
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => if let InputListState::Changed = self.state {},
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
        }
    }
}

impl Widget for InputList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self.create_list_items();
        let mut list = List::new(items);

        if self.show_border {
            let block = if let Some(title) = &self.title {
                Block::bordered().title(title.clone())
            } else {
                Block::bordered()
            };
            list = list.block(block);
        } else if let Some(title) = &self.title {
            let block = Block::default().title(title.clone());
            list = list.block(block);
        }

        list.render(area, buf);
    }
}

impl Widget for &InputList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = self.create_list_items();
        let mut list = List::new(items);

        if self.show_border {
            let block = if let Some(ref title) = self.title {
                Block::default().title(title.as_str()).borders(Borders::ALL)
            } else {
                Block::default().borders(Borders::ALL)
            };
            list = list.block(block);
        } else if let Some(ref title) = self.title {
            let block = Block::default().title(title.as_str());
            list = list.block(block);
        }

        list.render(area, buf);
    }
}

impl Default for InputList {
    fn default() -> Self {
        Self {
            title: None,
            state: InputListState::None,
            fields: Fields::default(),
            focus_idx: 0,
            show_border: false,
        }
    }
}
