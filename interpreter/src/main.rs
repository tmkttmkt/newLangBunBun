use std::io::{self, Write}; // Write を明示的にインポート
mod interpreter;
fn main() {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        interpreter::interpreter::interpreter(&input);
    }
}
