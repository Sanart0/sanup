pub trait InputType: Clone + ToString {
    type Value;

    fn parse_input(&mut self, c: char) -> bool;
    fn remove_last(&mut self) -> bool;
    fn value(&self) -> Self::Value;
    fn set_focus(&mut self, focus: bool);
}

#[derive(Clone, Default)]
pub struct StringField {
    focus: bool,
    value: String,
}

impl Into<StringField> for String {
    fn into(self) -> StringField {
        StringField {
            focus: false,
            value: self,
        }
    }
}

impl ToString for StringField {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl InputType for StringField {
    type Value = String;

    fn parse_input(&mut self, c: char) -> bool {
        if c.is_ascii() && !c.is_control() {
            self.value.push(c);
            true
        } else {
            false
        }
    }

    fn remove_last(&mut self) -> bool {
        self.value.pop().is_some()
    }

    fn value(&self) -> Self::Value {
        self.value.clone()
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}

#[derive(Clone, Default)]
pub struct IntegerField {
    focus: bool,
    value: i128,
}

impl IntegerField {
    pub fn value(&self) -> i128 {
        self.value
    }
}

impl Into<IntegerField> for i128 {
    fn into(self) -> IntegerField {
        IntegerField {
            focus: false,
            value: self,
        }
    }
}

impl ToString for IntegerField {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl InputType for IntegerField {
    type Value = i128;

    fn parse_input(&mut self, c: char) -> bool {
        if c.is_digit(10) {
            let new_value = self
                .value
                .saturating_mul(10)
                .saturating_add(c.to_digit(10).unwrap() as i128);
            if new_value <= i128::MAX {
                self.value = new_value;
                return true;
            }
        } else if c == '-' {
            self.value = self.value.saturating_mul(-1);
            return true;
        }
        false
    }

    fn remove_last(&mut self) -> bool {
        self.value /= 10;
        true
    }

    fn value(&self) -> Self::Value {
        self.value
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}

#[derive(Clone, Default)]
pub struct FloatField {
    focus: bool,
    value: f64,
    input: String,
    decimal_entered: bool,
}

impl Into<FloatField> for f64 {
    fn into(self) -> FloatField {
        FloatField {
            focus: false,
            value: self,
            input: self.to_string(),
            decimal_entered: true,
        }
    }
}

impl ToString for FloatField {
    fn to_string(&self) -> String {
        self.input.clone()
    }
}

impl InputType for FloatField {
    type Value = f64;

    fn parse_input(&mut self, c: char) -> bool {
        if c.is_digit(10) {
            self.input.push(c);
            self.value = self.input.parse().unwrap_or(self.value);
            true
        } else if c == '-' {
            if self.input.chars().nth(0) == Some('-') {
                self.input.remove(0);
            } else {
                self.input.insert(0, c);
            }
            self.value *= -1.0;
            true
        } else if c == '.' && !self.decimal_entered {
            self.input.push(c);
            self.decimal_entered = true;
            true
        } else {
            false
        }
    }

    fn remove_last(&mut self) -> bool {
        if let Some(last) = self.input.pop() {
            if last == '.' {
                self.decimal_entered = false;
            }
            self.value = self.input.parse().unwrap_or(0.0);
            true
        } else {
            false
        }
    }

    fn value(&self) -> Self::Value {
        self.value
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}
