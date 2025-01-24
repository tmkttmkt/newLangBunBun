// token/number_token.rs
use crate::token::ValueToken;

pub struct NumberToken {
    pub value: String,
}

impl ValueToken for NumberToken {
    fn is_match(&self, input: &str) -> bool {
        input.chars().all(|c| c.is_digit(10))
    }
    fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl NumberToken {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
