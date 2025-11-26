use crate::{
    app::{focus::SanupFocus, settings::Settings, tabs::SanupTabs},
    backup::task::BackupTask,
    ui::input::{Field, inputfield::InputField, inputform::InputForm},
};
use log::info;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    widgets::ListState,
};

pub struct Sanup {
    pub title: &'static str,
    pub focus: SanupFocus,
    pub tabs: SanupTabs,
    pub backups: Vec<BackupTask>,
    pub input_form: InputForm,
    pub settings: Settings,
    pub settings_list_state: ListState,
}

impl Sanup {
    pub fn on_key(&mut self, key: KeyEvent) {
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
                                        Field::Bool(InputField::new("BOOL")),
                                        Field::Integer(InputField::new("INTEGER")),
                                        Field::Float(InputField::new("FLOAT")),
                                        Field::String(InputField::new("STRING")),
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
                if self.input_form.is_submitted() {
                    //TODO Submitted Input Form
                    self.focus.to_body();
                } else if self.input_form.is_cancelled() {
                    //TODO Cancelled Input Form
                    self.focus.to_body();
                }

                self.input_form.on_key(key);
            }
        }
    }
}

impl Default for Sanup {
    fn default() -> Self {
        Sanup {
            title: "Sanup",
            focus: SanupFocus::Tabs,
            tabs: SanupTabs::Main,
            backups: Vec::new(),
            input_form: InputForm::empty(),
            settings: Settings::default(),
            settings_list_state: ListState::default().with_selected(None),
        }
    }
}
