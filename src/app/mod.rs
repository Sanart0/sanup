pub mod focus;
pub mod message;
pub mod state;
pub mod tabs;

use crate::{
    app::{focus::SanupFocus, message::Message, state::SanupState, tabs::SanupTabs},
    ui::{
        inputfield::InputField,
        inputform::{Field, InputForm},
    },
};
use log::info;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::sync::mpsc::{Receiver, Sender, channel};

pub struct Sanup {
    pub title: &'static str,
    pub state: SanupState,
    pub focus: SanupFocus,
    pub tabs: SanupTabs,
    pub backups: Vec<Receiver<Message>>,
    pub input_form: InputForm,
    pub body_text: String,
}

impl Sanup {
    pub fn on_key(&mut self, key: KeyEvent) {
        if let KeyCode::Esc = key.code {
            self.focus = SanupFocus::Tabs;
        }

        info!("APP EVENT: {:?}", key);

        match self.focus {
            SanupFocus::Tabs => {
                if let KeyCode::Char(c) = key.code {
                    match c {
                        'l' => self.tabs.next(),
                        'h' => self.tabs.prev(),
                        'j' => {
                            if self.focus.is_tabs() {
                                self.focus.to_body();
                            }
                        }
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
                if let KeyCode::Char('k') = key.code {
                    self.focus.to_tabs();
                }
            }
            SanupFocus::InputForm => {
                self.input_form.on_key(key);

                if self.input_form.is_submitted() {
                    //TODO Submitted Input Form
                    info!("SUBMITTED");
                    self.focus.to_body();
                } else if self.input_form.is_cancelled() {
                    //TODO Cancelled Input Form
                    info!("CANCELLED");
                    self.focus.to_body();
                }
            }
        }
    }
}

impl Default for Sanup {
    fn default() -> Self {
        Sanup {
            title: "Sanup",
            state: SanupState::None,
            focus: SanupFocus::Tabs,
            tabs: SanupTabs::Main,
            backups: Vec::new(),
            input_form: InputForm::empty(),
            body_text: String::new(),
        }
    }
}
