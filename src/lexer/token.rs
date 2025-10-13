#[derive(Debug, PartialEq)]
pub enum Token<> {
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

#[derive(Debug, PartialEq)]
pub enum Type {
  I32,
  F64,
}