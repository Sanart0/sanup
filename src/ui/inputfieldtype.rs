pub trait InputType: Clone + ToString {
    fn parse_input(&mut self, c: char) -> bool;
    fn remove_last(&mut self) -> bool;
}

#[derive(Clone, Default)]
pub struct StringField {
    value: String,
}

impl ToString for StringField {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl InputType for StringField {
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
}

#[derive(Clone, Default)]
pub struct IntegerField {
    value: i128,
}

impl ToString for IntegerField {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl InputType for IntegerField {
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
}

#[derive(Clone, Default)]
pub struct FloatField {
    value: f64,
    input: String,
    decimal_entered: bool,
}

impl FloatField {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            input: String::new(),
            decimal_entered: false,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl ToString for FloatField {
    fn to_string(&self) -> String {
        self.input.clone()
    }
}

impl InputType for FloatField {
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
}
