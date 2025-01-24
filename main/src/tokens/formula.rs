// token/string_token.rs
use crate::token::Token;

pub trait FormulaToken: Token {}


// 開き括弧トークン
struct LeftParen;

impl FormulaToken for LeftParen {
    fn is_match(&self, input: &str) -> bool {
        input == "("
    }

    fn get_value(&self) -> String {
        "(".to_string()
    }
}

// 閉じ括弧トークン
struct RightParen;

impl FormulaToken for RightParen {
    fn is_match(&self, input: &str) -> bool {
        input == ")"
    }

    fn get_value(&self) -> String {
        ")".to_string()
    }
}

// カンマトークン
struct Comma;

impl FormulaToken for Comma {
    fn is_match(&self, input: &str) -> bool {
        input == ","
    }

    fn get_value(&self) -> String {
        ",".to_string()
    }
}

// ダブルクォートトークン
struct Quote;

impl FormulaToken for Quote {
    fn is_match(&self, input: &str) -> bool {
        input == "\""
    }

    fn get_value(&self) -> String {
        "\"".to_string()
    }
}