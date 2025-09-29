pub trait InputType: Clone + ToString {
    type Value;

    fn parse_input(&mut self, c: char);
    fn remove_last(&mut self);
    fn value(&self) -> Self::Value;
    fn set_focus(&mut self, focus: bool);
}

#[derive(Clone, Default)]
pub struct StringField {
    focus: bool,
    value: String,
}

impl From<String> for StringField {
    fn from(value: String) -> Self {
        StringField {
            focus: false,
            value,
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for StringField {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl InputType for StringField {
    type Value = String;

    fn parse_input(&mut self, c: char) {
        if c.is_ascii() && !c.is_control() {
            self.value.push(c);
        }
    }

    fn remove_last(&mut self) {
        self.value.pop();
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

impl From<i128> for IntegerField {
    fn from(value: i128) -> Self {
        IntegerField {
            focus: false,
            value,
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for IntegerField {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl InputType for IntegerField {
    type Value = i128;

    fn parse_input(&mut self, c: char) {
        if c.is_ascii_digit() {
            self.value = self
                .value
                .saturating_mul(10)
                .saturating_add(c.to_digit(10).unwrap() as i128);
        } else if c == '-' {
            self.value = self.value.saturating_mul(-1);
        }
    }

    fn remove_last(&mut self) {
        self.value /= 10;
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

impl From<f64> for FloatField {
    fn from(value: f64) -> Self {
        FloatField {
            focus: false,
            value,
            input: value.to_string(),
            decimal_entered: true,
        }
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for FloatField {
    fn to_string(&self) -> String {
        self.input.clone()
    }
}

impl InputType for FloatField {
    type Value = f64;

    fn parse_input(&mut self, c: char) {
        if c.is_ascii_digit() {
            self.input.push(c);
            self.value = self.input.parse().unwrap_or(self.value);
        } else if c == '-' {
            if self.input.chars().nth(0) == Some('-') {
                self.input.remove(0);
            } else {
                self.input.insert(0, c);
            }
            self.value *= -1.0;
        } else if c == '.' && !self.decimal_entered {
            self.input.push(c);
            self.decimal_entered = true;
        }
    }

    fn remove_last(&mut self) {
        if let Some(last) = self.input.pop() {
            if last == '.' {
                self.decimal_entered = false;
            }
            self.value = self.input.parse().unwrap_or(0.0);
        }
    }

    fn value(&self) -> Self::Value {
        self.value
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}
