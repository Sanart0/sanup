use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum SanupFocus {
    Tabs,
    Body,
    InputForm,
}

impl SanupFocus {
    pub fn into_idx(&self) -> usize {
        match self {
            Self::Tabs => 0,
            Self::Body => 0,
            Self::InputForm => 0,
        }
    }

    pub fn is_tabs(&self) -> bool {
        *self == Self::Tabs
    }

    pub fn is_body(&self) -> bool {
        *self == Self::Body
    }

    pub fn is_inputform(&self) -> bool {
        *self == Self::InputForm
    }

    pub fn to_tabs(&mut self) {
        *self = Self::Tabs;
    }

    pub fn to_body(&mut self) {
        *self = Self::Body;
    }

    pub fn to_inputform(&mut self) {
        *self = Self::InputForm;
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
