pub mod message;
pub mod state;

use crate::sanup::{message::Message, state::SanupState};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct Sanup {
    pub state: SanupState,
    pub channel: (Sender<Message>, Receiver<Message>),
    backups: Vec<Receiver<Message>>,
}

impl Sanup {
    pub fn new() -> Self {
        Sanup {
            state: SanupState::Main,
            channel: channel(),
            backups: Vec::new(),
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if let KeyCode::Char(c) = key.code {
            match c {
                'h' => self.state.prev(),
                'l' => self.state.next(),
                _ => {}
            }
        }
    }
}
