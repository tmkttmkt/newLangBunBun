
pub trait Token {
    fn is_match(&self, input: &str) -> bool;
    fn get_value(&self) -> String;
}

pub trait ValueToken: Token {}
pub trait FormulaToken: Token {}