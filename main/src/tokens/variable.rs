use crate::token::ValueToken;

pub struct VariableToken {
    pub value: String,
}

impl  ValueToken for VariableToken {
    fn is_match(&self, input: &str) -> bool {
        input.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_')
    }
    fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl VariableToken {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
