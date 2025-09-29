pub mod focus;
pub mod message;
pub mod state;
pub mod tabs;

use crate::{
    sanup::{focus::SanupFocus, message::Message, state::SanupState, tabs::SanupTabs},
    ui::{
        inputfield::InputField,
        inputform::{Field, InputForm},
    },
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct Sanup {
    pub state: SanupState,
    pub title: &'static str,
    pub focus: SanupFocus,
    pub tabs: SanupTabs,
    pub channel: (Sender<Message>, Receiver<Message>),
    pub backups: Vec<Receiver<Message>>,
    pub input_form: InputForm,
}

impl Sanup {
    pub fn on_key(&mut self, key: KeyEvent) {
        if let KeyCode::Esc = key.code {
            self.focus = SanupFocus::Tabs;
        }

        match self.focus {
            SanupFocus::Tabs => {
                if let KeyCode::Char(c) = key.code {
                    match c {
                        'l' => self.tabs.next(),
                        'h' => self.tabs.prev(),
                        'c' => {
                            if self.tabs.is_backups() {
                                self.input_form = InputForm::new(
                                    "TEST INPUTFORM",
                                    vec![
                                        Field::String(InputField::new("SOME STRING")),
                                        Field::Integer(InputField::new("SOME INTEGER")),
                                        Field::Float(InputField::new("SOME FLOAT")),
                                    ],
                                );
                                self.focus.to_inputform();
                            }
                        }
                        _ => {}
                    }
                }
            }
            SanupFocus::Body => {
                if let KeyCode::Esc = key.code {
                    self.focus.to_tabs();
                }
            }
            SanupFocus::InputForm => {
                self.input_form.on_key(key);

                if self.input_form.is_submitted() {
                    //TODO Submitted Input Form
                    self.focus.to_body();
                } else if self.input_form.is_cancelled() {
                    //TODO Cancelled Input Form
                    self.focus.to_body();
                }
            }
        }
    }
}

impl Default for Sanup {
    fn default() -> Self {
        Sanup {
            state: SanupState::None,
            title: "Sanup",
            focus: SanupFocus::Tabs,
            tabs: SanupTabs::Main,
            channel: channel(),
            backups: Vec::new(),
            input_form: InputForm::empty(),
        }
    }
}
