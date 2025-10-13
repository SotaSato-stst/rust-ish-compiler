use core::str;

use super::tokenizer;
use super::lexer;

pub fn build_ast(source_text: &str) {
  let _tokens = lex(&source_text);
}

fn lex(source_string: &str) {
  let chunks = tokenizer::to_token_chunks(source_string).into_iter().peekable();
  let tokens = lexer::to_token_stream(chunks.collect());
  println!("{:?}", tokens);
}

fn parse(tokens: Vec<super::token::Token>) {
// TODO: implement parsing logic here
}

enum Item {
  ItemFn(ItemFn),
  ItemConst(ItemConst),
}

struct ItemFn {
  signature: FnSignature,
  block: Vec<Statement>,
}

enum Statement {
  Local(Local)
}

struct Local {
  name: String,
  var_type: String,
  value: Expr,
}

enum Expr {
  ExprLit(String),
  ExprBinaryOp {
    left: Box<Expr>,
    op: String,
    right: Box<Expr>,
  },
  ExprVariable(String),
}

struct FnSignature {
  ident: String,
  args: Vec<FnArg>,
  output: Option<String>,
}

struct FnArg {
  name: String,
  arg_type: String,
}

struct ItemConst {
  name: String,
  value: String,
}

pub struct Program {
  pub items: Vec<Item>,
}

#[cfg(test)]
pub mod tests {
    use crate::lexer::token::{Token, Type};
    use crate::libs;

    use super::*;
    #[test]
    fn for_test() {
        let filename = "./src/lexer/test/sample.txt";
        let source_code = libs::readfile(filename);
        build_ast(source_code.as_str());
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
