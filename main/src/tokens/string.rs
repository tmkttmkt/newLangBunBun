use crate::token::ValueToken;

pub struct StringToken {
    pub value: String,
}

impl ValueToken for StringToken {
    fn is_match(&self, input: &str) -> bool {
        input.starts_with('"') && input.ends_with('"')
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl StringToken {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
