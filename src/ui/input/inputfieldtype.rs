use ratatui::crossterm::event::KeyEvent;

use crate::ui::input::inputfield::InputFieldKind;

pub trait InputType: Default + ToString {
    type Value;

    fn kind() -> InputFieldKind;
    fn value(&self) -> Self::Value;
    fn set_focus(&mut self, focus: bool);
    fn on_key(&mut self, key: KeyEvent);
}
