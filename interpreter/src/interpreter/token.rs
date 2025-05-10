#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Dot,
    DoubleQuotes,
    LeftParen,
    RightParen,
    LeftDicParen,
    RightDicParen,
    Line,
    Assignment,
    RangeComment,
    Comment,
    EOF,
}
pub fn match_token(i: &str) -> (&str, Option<Token>) {
    if let (i, Some(ident_res)) = check_ident(whitespace(i)) {
        return (i, Some(ident_res));
    }
    
    if let (i, Some(number_res)) = check_number(whitespace(i)) {
        return (i, Some(number_res));
    }
    if let (i, Some(number_res)) = check_comment(whitespace(i)) {
        return (i, Some(number_res));
    }
    
    if let (i, Some(lparen_res)) = check_char(whitespace(i)) {
        return (i, Some(lparen_res));
    }

    (whitespace(i), None)
}
fn whitespace(mut input: &str) -> &str {
  while matches!(peek_char(input), Some(' ')) {
    input = advance_char(input);
  }
  input
}

fn check_ident(mut input: &str) -> (&str, Option<Token>) {
    let mut value_str = String::new();
    if matches!(
      peek_char(input),
      Some(_x @ ('a'..='z' | 'A'..='Z'))
    ) {
        value_str.push(peek_char(input).unwrap());
      input = advance_char(input);
      while matches!(
        peek_char(input),
        Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9'))
      ) {
        value_str.push(peek_char(input).unwrap());
        input = advance_char(input);
      }
      (input, Some(Token::Ident(value_str.parse().unwrap())))
    } else {
      (input, None)
    }
  }
  
  fn check_number(mut input: &str) -> (&str, Option<Token>) {
    let mut value_str = String::new();
    if matches!(
      peek_char(input),
      Some(_x @ ('0'..='9'))
    ) {
        value_str.push(peek_char(input).unwrap());
      input = advance_char(input);
      while matches!(
        peek_char(input),
        Some(_x @ ('.' | '0'..='9'))
      ) {
        value_str.push(peek_char(input).unwrap());
        input = advance_char(input);
      }
      (input, Some(Token::Number(value_str.parse::<i64>().unwrap())))
    } else {
      (input, None)
    }
  }
fn check_char(input: &str) -> (&str, Option<Token>) {
    let ch=peek_char(input).unwrap();
    match ch {
        '+' => (advance_char(input), Some(Token::Plus)),
        '-' => (advance_char(input), Some(Token::Minus)),
        '*' => (advance_char(input), Some(Token::Multiply)),
        '/' => (advance_char(input), Some(Token::Divide)),
        ',' => (advance_char(input), Some(Token::Dot)),
        '"' => (advance_char(input), Some(Token::DoubleQuotes)),
        '\n' => (advance_char(input), Some(Token::Line)),
        '=' => (advance_char(input), Some(Token::Assignment)),
        '(' => (advance_char(input), Some(Token::LeftParen)),
        ')' => (advance_char(input), Some(Token::RightParen)),
        '{' => (advance_char(input), Some(Token::LeftDicParen)),
        '}' => (advance_char(input), Some(Token::RightDicParen)),
        _ => (advance_char(input), None),
    }
}
fn check_comment(mut input: &str) -> (&str, Option<Token>) {
    if matches!(
      peek_char(input),
      Some('#')
    ) {
        input = advance_char(input);
        if matches!(
          peek_char(input),
          Some('#')
        ) {
            (input, Some(Token::RangeComment))
        } else {
            (input, Some(Token::Comment))
        }
    } else {
      (input, None)
    }
}
fn advance_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

fn peek_char(input: &str) -> Option<char> {
    input.chars().next()
  }
