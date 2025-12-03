use std::fmt::Debug;

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

#[derive(Clone)]
pub enum EmptyEnum {
    Empty,
}

impl EnumVariants for EmptyEnum {
    fn default(&self) -> Box<dyn EnumVariants> {
        Box::new(EmptyEnum::Empty)
    }

    fn longest(&self) -> String {
        String::new()
    }

    fn variants(&self) -> Vec<String> {
        Vec::new()
    }

    fn from_string(&self, _s: String) -> Box<dyn EnumVariants> {
        Self::default(&self)
    }

    fn clone_box(&self) -> Box<dyn EnumVariants> {
        Box::new(self.clone())
    }
}

impl ToString for EmptyEnum {
    fn to_string(&self) -> String {
        String::new()
    }
}
