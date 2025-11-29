use std::{fmt::Debug, sync::OnceLock};

pub trait EnumVariants: ToString + Send + Sync {
    fn default(&self) -> Box<dyn EnumVariants>;
    fn longest(&self) -> String;
    fn variants(&self) -> Vec<String>;
    fn from_string(&self, s: String) -> Box<dyn EnumVariants>;
    fn clone_box(&self) -> Box<dyn EnumVariants>;
}

impl Clone for Box<dyn EnumVariants> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl ToString for Box<dyn EnumVariants> {
    fn to_string(&self) -> String {
        (**self).to_string()
    }
}

impl Debug for Box<dyn EnumVariants> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub static DEFAULT_ENUM: OnceLock<Box<dyn EnumVariants>> = OnceLock::new();

pub fn set_default_enum_variant(variant: Box<dyn EnumVariants>) {
    DEFAULT_ENUM.set(variant).ok();
}
