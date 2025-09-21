use std::fmt::Display;

pub enum SanupTabs {
    Settings,
    Main,
    Processes,
    Backups,
}

impl SanupTabs {
    pub fn into_vec_str() -> Vec<String> {
        vec![
            Self::Settings.to_string(),
            Self::Main.to_string(),
            Self::Processes.to_string(),
            Self::Backups.to_string(),
        ]
    }

    pub fn into_idx(&self) -> usize {
        match self {
            Self::Settings => 0,
            Self::Main => 1,
            Self::Processes => 3,
            Self::Backups => 2,
        }
    }

    pub fn next(&mut self) {
        match self {
            Self::Settings => *self = Self::Main,
            Self::Main => *self = Self::Backups,
            Self::Processes => *self = Self::Settings,
            Self::Backups => *self = Self::Processes,
        }
    }

    pub fn prev(&mut self) {
        match self {
            Self::Settings => *self = Self::Processes,
            Self::Main => *self = Self::Settings,
            Self::Processes => *self = Self::Backups,
            Self::Backups => *self = Self::Main,
        }
    }
}

impl Display for SanupTabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Settings => "Settings",
                Self::Main => "Main",
                Self::Processes => "Processes",
                Self::Backups => "Backups",
            }
        )
    }
}
