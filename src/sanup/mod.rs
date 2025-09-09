pub mod state;

use crate::sanup::state::SanupState;

pub struct Sanup {
    pub state: SanupState,
}

impl Sanup {
    pub fn new() -> Self {
        Sanup {
            state: SanupState::Main,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'h' => self.state.prev(),
            'l' => self.state.next(),
            _ => {}
        }
    }
}
