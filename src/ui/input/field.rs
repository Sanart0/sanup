use crate::ui::input::{
    boolfield::BoolField, enumfield::EnumField, floatfield::FloatField, inputfield::InputField,
    integerfield::IntegerField, stringfield::StringField,
};
use std::{
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

#[derive(Clone)]
pub enum Field {
    Bool(InputField<BoolField>),
    Integer(InputField<IntegerField>),
    Float(InputField<FloatField>),
    String(InputField<StringField>),
    Enum(InputField<EnumField>),
}

#[derive(Default)]
pub struct Fields(Vec<Field>);

impl Fields {
    pub fn new(values: Vec<Field>) -> Self {
        Self(values)
    }
}

impl IntoIterator for Fields {
    type Item = Field;

    type IntoIter = IntoIter<Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Fields {
    type Item = &'a Field;
    type IntoIter = Iter<'a, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Fields {
    type Item = &'a mut Field;
    type IntoIter = IterMut<'a, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Into<Fields> for Vec<Field> {
    fn into(self) -> Fields {
        Fields(self)
    }
}

impl Deref for Fields {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
