use std::fmt::Display;

pub enum SanupState {
    Settings,
    Main,
    Backups,
    Processes,
}

impl SanupState {
    pub fn into_vec_str() -> Vec<String> {
        vec![
            Self::Settings.to_string(),
            Self::Main.to_string(),
            Self::Backups.to_string(),
            Self::Processes.to_string(),
        ]
    }

    pub fn into_idx(&self) -> usize {
        match self {
            Self::Settings => 0,
            Self::Main => 1,
            Self::Backups => 2,
            Self::Processes => 3,
        }
    }

    pub fn next(&mut self) {
        match self {
            Self::Settings => *self = Self::Main,
            Self::Main => *self = Self::Backups,
            Self::Backups => *self = Self::Processes,
            Self::Processes => *self = Self::Settings,
        }
    }

    pub fn prev(&mut self) {
        match self {
            Self::Settings => *self = Self::Processes,
            Self::Main => *self = Self::Settings,
            Self::Backups => *self = Self::Main,
            Self::Processes => *self = Self::Backups,
        }
    }
}

impl Display for SanupState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Settings => "Settings",
                Self::Main => "Main",
                Self::Backups => "Backups",
                Self::Processes => "Processes",
            }
        )
    }
}
