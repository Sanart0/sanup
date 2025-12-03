use crate::ui::input::{
    enumvariants::{EmptyEnum, EnumVariants},
    inputfield::InputFieldKind,
    inputfieldtype::InputType,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Margin, Rect},
    widgets::{Block, Borders, Clear, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumFieldState {
    Hidden,
    Active,
}

#[derive(Clone)]
pub struct EnumField {
    state: EnumFieldState,
    focus: bool,
    value: Box<dyn EnumVariants>,
    variants: Vec<String>,
    selected_idx: usize,
}

impl EnumField {
    pub fn state(&self) -> EnumFieldState {
        self.state
    }

    pub fn set_state(&mut self, state: EnumFieldState) {
        self.state = state;
    }

    pub fn is_active(&self) -> bool {
        self.state == EnumFieldState::Active
    }

    pub fn value(&self) -> &dyn EnumVariants {
        &*self.value
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl InputType for EnumField {
    type Value = Box<dyn EnumVariants>;

    fn kind() -> InputFieldKind {
        InputFieldKind::Enum
    }

    fn value(&self) -> Self::Value {
        self.value.clone_box()
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
        if focus {
            self.selected_idx = self
                .variants
                .iter()
                .position(|v| v.eq(&self.value.to_string()))
                .unwrap_or(0);
        }
    }

    fn on_key(&mut self, key: KeyEvent) {
        if self.is_active() {
            match key.code {
                KeyCode::Esc => {
                    self.value = self.value.default();
                    self.set_state(EnumFieldState::Hidden);
                }
                KeyCode::Enter => {
                    self.value = {
                        if let Some(value) = self.variants.get(self.selected_idx) {
                            self.value.from_string(value.clone())
                        } else {
                            self.value.default()
                        }
                    };
                    self.set_state(EnumFieldState::Hidden);
                }
                KeyCode::Char('k') => {
                    if self.selected_idx == 0 {
                        self.selected_idx += self.variants.len();
                    }
                    self.selected_idx -= 1;
                }
                KeyCode::Char('j') => {
                    self.selected_idx += 1;
                    self.selected_idx %= self.variants.len();
                }
                _ => {}
            }
        } else if let KeyCode::Char('a') = key.code {
            self.set_state(EnumFieldState::Active);
        }
    }
}

impl Widget for EnumField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.is_active() {
            let height = self.variants.len() as u16 + 2;
            let popup_area = Rect {
                x: area.x,
                y: area.y - height,
                width: self.value.longest().len() as u16 + 4,
                height,
            };

            Clear.render(popup_area, buf);

            let block = Block::default().title("Variants").borders(Borders::ALL);
            block.render(popup_area, buf);

            let inner_area = popup_area.inner(Margin::new(1, 1));
            let constraints: Vec<Constraint> = (0..self.variants.len())
                .map(|_| Constraint::Length(1))
                .collect();
            let layout = Layout::vertical(constraints);
            let field_areas = layout.split(inner_area);

            for (i, variant) in self.variants.iter().enumerate() {
                let area = field_areas[i];
                let variant = {
                    if i == self.selected_idx {
                        format!("*{}", variant)
                    } else {
                        format!(" {}", variant)
                    }
                };
                variant.render(area, buf);
            }
        } else {
            self.value.to_string().render(area, buf);
        }
    }
}

impl<T> From<T> for EnumField
where
    T: EnumVariants + 'static,
{
    fn from(value: T) -> Self {
        let value: Box<dyn EnumVariants> = Box::new(value);

        let variants = value.variants();
        let selected_idx = variants
            .iter()
            .position(|v| v.eq(&value.to_string()))
            .unwrap_or(0);

        EnumField {
            state: EnumFieldState::Hidden,
            focus: false,
            value,
            variants,
            selected_idx,
        }
    }
}

impl ToString for EnumField {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Default for EnumField {
    fn default() -> Self {
        Self::from(EmptyEnum::Empty)
    }
}
