use super::token::Token;
use super::token::match_token;

pub fn lexer(mut input: &str) -> Result<Vec<Token>, String> {

    let mut tokens = vec![];
    while !input.is_empty() {
      input = if let (next_input, Some(token)) = match_token(input) {
        tokens.push(token);
        next_input
      } else {
        return Err(format!("Unexpected character: {}", input.chars().next().unwrap()));
      }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
}
pub fn parser(tokens: &[Token]) {
    let tokens = tokens.iter().peekable();
    // ここに構文解析のロジックを追加します
}

pub fn interpreter(line: &str) {
    match lexer(line) {
        Ok(tokens) => {
            parser(&tokens);
            println!("Parsed tokens: {:?}", tokens);
        },
        Err(e) => println!("Error: {}", e),
    }
}
