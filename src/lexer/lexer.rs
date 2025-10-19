use super::token::*;

pub fn to_token_stream(token_chunks: Vec<String>) -> Vec<Token> {
  let mut tokens = Vec::<Token>::new();
  let mut chunks = token_chunks.into_iter().peekable();

  while chunks.peek() != None {
    let chunk = chunks.next().unwrap();
    match chunk.as_str() {
      " " => continue,
      "{" => tokens.push(Token::LBrace),
      "}" => tokens.push(Token::RBrace),
      "(" => tokens.push(Token::LParentheses),
      ")" => tokens.push(Token::RParentheses),
      "[" => tokens.push(Token::LBracket),
      "]" => tokens.push(Token::RBracket),
      "<" => tokens.push(Token::LAngleBracket),
      ">" => tokens.push(Token::RAngleBracket),
      ":" => tokens.push(Token::Collon),
      ";" => tokens.push(Token::Semicolon),
      "," => tokens.push(Token::Comma),
      "fn" => tokens.push(Token::Fn),
      "let" => tokens.push(Token::Let),
      "const" => tokens.push(Token::Const),
      "i32" => tokens.push(Token::Type(Type::I32)),
      "f64" => tokens.push(Token::Type(Type::F64)),
      _ => tokens.push(Token::Identifier(chunk)),
    }
  } 
  tokens
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenizer;
    use crate::lexer::token::{Token, Type};
    use super::*;

    #[test]
    fn test_lex() {
    let source = "fn main() { let x: i32 = 10; }";
    let chunks = tokenizer::to_token_chunks(source);
    let tokens = to_token_stream(chunks);
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
        Token::RBrace,
    ];
    assert_eq!(tokens, expected_tokens);
  }
}

