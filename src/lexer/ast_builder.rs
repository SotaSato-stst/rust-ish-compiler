use std::{fs::File, io::{BufRead, BufReader}};
use super::tokenizer;
use super::lexer;
use super::token::*;

pub fn build_ast(file_name: &str) {
  let file = File::open(file_name).expect("Could not open file");
  let reader = BufReader::new(file);

  let mut lines = Vec::<String>::new();

  for line in reader.lines() {
      match line {
          Ok(content) => lines.push(content),
          Err(e) => eprintln!("Error reading line: {}", e),
      }
  }

  let source = lines.join("");
  let _tokens = lex(&source);
}


fn lex(source_string: &str) {
  let chunks = tokenizer::to_token_chunks(source_string).into_iter().peekable();
  let tokens = lexer::to_token_stream(chunks.collect());
  println!("{:?}", tokens);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn for_test() {
        let filename = "./src/lexer/test/sample.txt";
        build_ast(filename);
    }

    #[test]
    fn test_lex() {
    let source = "fn main() { let x: i32 = 10; }";
    let chunks = tokenizer::to_token_chunks(source);
    let tokens = lexer::to_token_stream(chunks);
    let expected_tokens = vec![
        Token::Fn,
        Token::Identifier("main".to_string()),
        Token::LParentheses,
        Token::RParentheses,
        Token::LBrace,
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Collon,
        Token::Type(Type::I32),
        Token::Identifier("=".to_string()),
        Token::Identifier("10".to_string()),
        Token::Semicolon,
        Token::RBrace(),
    ];
    assert_eq!(tokens, expected_tokens);
  }
}
