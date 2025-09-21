pub mod focus;
pub mod message;
pub mod state;
pub mod tabs;

use crate::{
    sanup::{focus::SanupFocus, message::Message, state::SanupState, tabs::SanupTabs},
    ui::{inputfield::InputField, inputfieldtype::StringField, inputform::{Field, InputForm}},
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct Sanup {
    pub state: SanupState,
    pub focus: SanupFocus,
    pub tabs: SanupTabs,
    pub channel: (Sender<Message>, Receiver<Message>),
    pub backups: Vec<Receiver<Message>>,
    pub input_form: InputForm,
}

impl Sanup {
    pub fn new() -> Self {
        Sanup {
            state: SanupState::None,
            focus: SanupFocus::Tabs,
            tabs: SanupTabs::Main,
            channel: channel(),
            backups: Vec::new(),
            input_form: InputForm::new(vec![
                Field::String(InputField::new("SOME STRING")),
                Field::Integer(InputField::new("SOME INTEGER")),
                Field::Float(InputField::new("SOME FLOAT")),
            ])
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if let KeyCode::Esc = key.code {
            self.focus = SanupFocus::Tabs;
        }

        match self.focus {
            SanupFocus::Tabs => {
                if let KeyCode::Char(c) = key.code {
                    match c {
                        'h' => self.tabs.prev(),
                        'l' => self.tabs.next(),
                        'i' => self.focus = SanupFocus::InputForm,
                        _ => {}
                    }
                }
            }
            SanupFocus::Body => todo!(),
            SanupFocus::InputForm => {
                self.input_form.on_key(key);
            },
        }
    }
}
