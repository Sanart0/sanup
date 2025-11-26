use crate::ui::input::{
    boolfield::BoolField, floatfield::FloatField, inputfield::InputField,
    integerfield::IntegerField, stringfield::StringField,
};

pub mod boolfield;
pub mod enumfield;
pub mod floatfield;
pub mod inputfield;
pub mod inputfieldtype;
pub mod inputform;
pub mod integerfield;
pub mod stringfield;

#[derive(Clone)]
pub enum Field {
    Bool(InputField<BoolField>),
    Integer(InputField<IntegerField>),
    Float(InputField<FloatField>),
    String(InputField<StringField>),
}

pub enum Value {
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
}
