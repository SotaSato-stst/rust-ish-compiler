use std::{fs::File, io::{BufRead, BufReader}};

fn lexer(file_name: &str) {
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

fn is_token_separator(c: char) -> bool {
  match c {
    ' ' | ':' | ';' | '[' | ']' | '{' | '}' | '(' | ')' | '<' | '>' | ',' => true,
    _ => false,
  }
}

fn to_token_chunks(source_string: &str) -> Vec<String> {
  let mut char_chunks = Vec::<String>::new();
  let mut chunk_buffer = Vec::<char>::new();
  let mut chars = source_string.chars().peekable();
  let mut char = chars.next().unwrap();

  while chars.peek() != None {
    if is_token_separator(char) {
      if !chunk_buffer.is_empty() {
        char_chunks.push(chunk_buffer.iter().collect());
        chunk_buffer.clear();
      }
      char_chunks.push(char.to_string());
    } else {
      chunk_buffer.push(char);
    }
    char = chars.next().unwrap();
  }

  char_chunks
}

fn to_token_stream(token_chunks: Vec<String>) -> Vec<Token> {
  let mut tokens = Vec::<Token>::new();
  let mut chunks = token_chunks.into_iter().peekable();

  while chunks.peek() != None {
    let chunk = chunks.next().unwrap();
    match chunk.as_str() {
      " " => continue,
      "{" => tokens.push(Token::LBrace),
      "}" => tokens.push(Token::RBrace()),
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

fn lex(source_string: &str) {
  let mut chunks = to_token_chunks(source_string).into_iter().peekable();
  let tokens = to_token_stream(chunks.collect());
  println!("{:?}", tokens);
}

#[derive(Debug)]
enum Token<> {
  RBrace(),
  LBrace,
  RParentheses,
  LParentheses,
  RBracket,
  LBracket,
  RAngleBracket,
  LAngleBracket,
  Fn,
  Collon,
  Semicolon,
  Comma,
  Identifier(String),
  Let,
  Const,
  Type(Type)
}

#[derive(Debug)]
enum Type {
  I32,
  F64,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let filename = "./src/lexer/test/sample.txt";
        lexer(filename);
    }
}

