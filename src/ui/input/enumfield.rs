use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::ui::input::inputfieldtype::InputType;

pub trait EnumVariants: ToString + Sized + Clone + Eq {
    fn variants() -> Vec<Self>;
    fn display_name(&self) -> String;
}

#[derive(Clone, Default)]
pub struct EnumField<E: EnumVariants> {
    focus: bool,
    pub value: E,
    variants: Vec<E>,
    pub input: String,
    pub selected_index: usize,
}

impl<E: EnumVariants> EnumField<E> {
    pub fn matching_variants(&self) -> Vec<&E> {
        if self.input.is_empty() {
            self.variants.iter().collect()
        } else {
            self.variants
                .iter()
                .filter(|v| {
                    v.to_string()
                        .to_lowercase()
                        .starts_with(&self.input.to_lowercase())
                })
                .collect()
        }
    }

    pub fn select_current(&mut self) {
        let selected = self
            .matching_variants()
            .get(self.selected_index)
            .cloned()
            .cloned()
            .unwrap_or(self.value.clone());

        self.value = selected.clone();
        self.input = selected.to_string();
    }

    pub fn parse_input(&mut self, c: char) {
        if !c.is_control() && c.is_ascii() {
            self.input.push(c);
            self.selected_index = 0;
            let matching = self.matching_variants();
            if let Some(&first_match) = matching.first() {
                self.value = first_match.clone();
            }
        }
    }

    pub fn remove_last(&mut self) {
        self.input.pop();
        self.selected_index = 0;
        let matching = self.matching_variants();
        if let Some(&first) = matching.first() {
            self.value = first.clone();
        }
    }
}

impl<E: EnumVariants> From<E> for EnumField<E> {
    fn from(value: E) -> Self {
        let variants = E::variants();
        let selected_index = variants
            .iter()
            .position(|v| std::mem::discriminant(v) == std::mem::discriminant(&value))
            .unwrap_or(0);

        EnumField {
            focus: false,
            value: value.clone(),
            input: value.display_name(),
            selected_index,
            variants,
        }
    }
}

impl<E: EnumVariants> ToString for EnumField<E> {
    fn to_string(&self) -> String {
        if self.focus && !self.input.is_empty() {
            self.input.clone()
        } else {
            self.value.display_name()
        }
    }
}

impl<E: EnumVariants> InputType for EnumField<E> {
    type Value = E;

    fn value(&self) -> Self::Value {
        self.value.clone()
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
        if focus {
            self.input = self.value.to_string();
            self.selected_index = self
                .variants
                .iter()
                .position(|v| v == &self.value)
                .unwrap_or(0);
        } else {
            self.input.clear();
        }
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let matching = self.matching_variants();
                if !matching.is_empty() {
                    self.selected_index = self.selected_index.saturating_sub(1);
                    self.select_current();
                }
            }
            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let matching = self.matching_variants();
                if !matching.is_empty() {
                    self.selected_index = (self.selected_index + 1) % matching.len();
                    self.select_current();
                }
            }
            KeyCode::Char(c) => self.parse_input(c),
            KeyCode::Backspace => self.remove_last(),
            KeyCode::Enter | KeyCode::Tab => {
                self.select_current();
                self.input = self.value.to_string();
            }
            KeyCode::Esc => {
                self.input = self.value.to_string();
            }
            _ => {}
        }
    }
}
