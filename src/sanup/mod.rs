pub enum SanupState {
    Main,
}

pub struct Sanup {
    state: SanupState,
}

impl Sanup {
    pub fn new() -> Self {
        Sanup {
            state: SanupState::Main,
        }
    }

    pub fn on_key(&mut self, c: char) {}
}
