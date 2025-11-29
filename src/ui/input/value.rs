use crate::ui::input::enumvariants::EnumVariants;
use serde::{Serialize, Serializer, ser::SerializeMap};
use std::{
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

#[derive(Debug)]
pub enum Value {
    Bool(String, bool),
    Integer(String, i64),
    Float(String, f64),
    String(String, String),
    Enum(String, Box<dyn EnumVariants>),
}

#[derive(Default)]
pub struct Values(Vec<Value>);

impl Values {
    pub fn new(values: Vec<Value>) -> Self {
        Self(values)
    }
}

impl Serialize for Values {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for value in &self.0 {
            match value {
                Value::Bool(name, b) => map.serialize_entry(name, b)?,
                Value::Integer(name, i) => map.serialize_entry(name, i)?,
                Value::Float(name, f) => map.serialize_entry(name, f)?,
                Value::String(name, s) => map.serialize_entry(name, s)?,
                Value::Enum(name, e) => map.serialize_entry(name, &e.to_string())?,
            }
        }
        map.end()
    }
}

impl IntoIterator for Values {
    type Item = Value;

    type IntoIter = IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Values {
    type Item = &'a Value;
    type IntoIter = Iter<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Values {
    type Item = &'a mut Value;
    type IntoIter = IterMut<'a, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl FromIterator<Value> for Values {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Values(iter.into_iter().collect())
    }
}

impl Deref for Values {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Values {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
