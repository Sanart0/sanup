use std::fmt::Display;

pub enum SanupFocus {
    Tabs,
    Body,
    InputForm,
}

impl SanupFocus {
    pub fn into_idx(&self) -> usize {
        match self {
            SanupFocus::Tabs => 0,
            SanupFocus::Body => 0,
            SanupFocus::InputForm => 0,
        }
    }
}

impl Display for SanupFocus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Tabs => "Tabs",
                Self::Body => "Body",
                Self::InputForm => "InputForm",
            }
        )
    }
}
