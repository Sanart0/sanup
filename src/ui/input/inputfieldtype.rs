use ratatui::crossterm::event::KeyEvent;

pub trait InputType: ToString {
    type Value;

    fn value(&self) -> Self::Value;
    fn set_focus(&mut self, focus: bool);
    fn on_key(&mut self, key: KeyEvent);
}
