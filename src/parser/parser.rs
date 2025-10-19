use super::lexer;
use super::token::Token;
use super::chunker;
use crate::ast::program::{Program, Item, ItemFn, FnSignature, FnParams, Statement, Local, FnCall, Expr};

pub fn parse(source_code: &String) -> Program {
  let chunks = chunker::to_token_chunks(source_code);
  let token_iter = lexer::to_token_stream(chunks);
    parse_to_program(token_iter)
}

fn parse_to_program(tokens: Vec<Token>) -> Program {
  let mut token_iter = tokens.into_iter().peekable();
  let mut items = Vec::<Item>::new();
  while token_iter.peek() != None {
    items.push(parse_item(&mut token_iter));
  }
    Program {
        items,
    }
}

fn parse_item(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>) -> Item {
  match token_iter.next().unwrap() {
    Token::Fn => return parse_item_fn(token_iter),
    _ => panic!("Unexpected token"),
  }
}

fn parse_item_fn(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>) -> Item {
  let signature = parse_fn_signature(token_iter);
  let block = parse_block(token_iter);
  Item::ItemFn(ItemFn {
    signature,
    block,
  })
}

fn parse_block(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>) -> Vec<Statement> {
  match token_iter.next().unwrap() {
    Token::LBrace => (),
    _ => panic!("Expected '{{'"),
  };
  let mut statements = Vec::<Statement>::new();
  while token_iter.peek() != None {
    match token_iter.peek().unwrap() {
      Token::RBrace => {
        token_iter.next();
        break;
      },
      _ => statements.push(parse_statement(token_iter)),
    }
  }
  statements
}

fn parse_statement(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>) -> Statement {
  match token_iter.next().unwrap() {
    Token::Let => {
        parse_let_statement(token_iter)
    },
    Token::Identifier(indent) => {
        let _indent = indent.clone();
        match token_iter.peek().unwrap() {
            Token::LParentheses => {
                parse_fn_call_statement(token_iter, &_indent)
            },
            _ => panic!("Unexpected token: {:?}", token_iter.peek().unwrap()),
        }
    }
    _ => panic!("Unexpected token in statement"),
  }
}

fn parse_fn_call_statement(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>, ident: &String) -> Statement {
    let args = parse_fn_arg(token_iter);
    match token_iter.next().unwrap() {
        Token::Semicolon => (),
        _ => panic!("Expected ';'"),
    };
    Statement::FnCall(FnCall {
        name: ident.clone(),
        args: args,
    })
}

fn parse_fn_arg(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>) -> Vec<Expr> {
    match token_iter.next().unwrap() {
        Token::LParentheses => (),
        _ => panic!("Expected '(': {:?}", token_iter.peek().unwrap()),
    };
    let mut args = Vec::<Expr>::new();
    while token_iter.peek() != None {
        match token_iter.peek().unwrap() {
            Token::RParentheses => {
                token_iter.next();
                break;
            },
            Token::Identifier(lit) => {
                if lit.starts_with("\"") {
                    if !lit.ends_with("\"") {
                        panic!("Unclosed string literal: {}", lit);
                    }
                    let unquoted = lit.trim_matches('"').to_string();
                    args.push(Expr::ExprLit(unquoted));
                } else {
                    args.push(Expr::ExprVariable(lit.clone()));
                }
                token_iter.next();
            },
            _ => panic!("Unexpected token in function arguments: {:?}", token_iter.peek().unwrap()),
        }
    }
    args
}

fn parse_let_statement(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>) -> Statement {
      let name = match token_iter.next().unwrap() {
        Token::Identifier(name) => name,
        _ => panic!("Expected identifier"),
      };
      match token_iter.next().unwrap() {
        Token::Collon => (),
        _ => panic!("Expected ':'"),
      };
      let var_type = match token_iter.next().unwrap() {
        Token::Type(t) => format!("{:?}", t),
        _ => panic!("Expected type"),
      };
      match token_iter.next().unwrap() {
        Token::Identifier(op) if op == "=" => (),
        _ => panic!("Expected '='"),
      };
      let value = match token_iter.next().unwrap() {
        Token::Identifier(lit) => Expr::ExprLit(lit),
        _ => panic!("Expected literal"),
      };
      match token_iter.next().unwrap() {
        Token::Semicolon => (),
        _ => panic!("Expected ';'"),
      };
      Statement::Local(Local {
        name,
        var_type,
        value,
      })
}

fn parse_fn_signature(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>) -> FnSignature {
  let ident = match token_iter.next().unwrap() {
    Token::Identifier(name) => name,
    _ => panic!("Expected function name"),
  };
  match token_iter.next().unwrap() {
    Token::LParentheses => (),
    _ => panic!("Expected '('"),
  };
  let mut args = Vec::<FnParams>::new();
  while token_iter.peek() != None {
    match token_iter.peek().unwrap() {
      Token::RParentheses => {
        token_iter.next();
        break;
      },
      _ => args.push(parse_fn_params(token_iter)),
    }
  }
  // TODO: コロンじゃなくて、-> であるべき
  let output = if let Some(Token::Collon) = token_iter.peek() {
    token_iter.next();
    match token_iter.next().unwrap() {
      Token::Type(t) => Some(format!("{:?}", t)),
      _ => panic!("Expected return type"),
    }
  } else {
    None
  };
  FnSignature {
    ident,
    args,
    output,
  }
}

fn parse_fn_params(token_iter: &mut core::iter::Peekable<impl Iterator<Item=Token>>) -> FnParams {
  let name = match token_iter.next().unwrap() {
    Token::Identifier(name) => name,
    _ => panic!("Expected argument name"),
  };
  match token_iter.next().unwrap() {
    Token::Collon => (),
    _ => panic!("Expected ':'"),
  };
  let arg_type = match token_iter.next().unwrap() {
    Token::Type(t) => format!("{:?}", t),
    _ => panic!("Expected argument type"),
  };
  if let Some(Token::Comma) = token_iter.peek() {
    token_iter.next();
  }
  FnParams {
    name,
    arg_type,
  }
}

#[cfg(test)]
mod tests {
  use crate::parser::token::{Token, Type};
  use super::*;
  use crate::libs;

    #[test]
    fn for_test() {
        let filename = "./src/parser/test/sample.txt";
        let source_code = libs::readfile(filename);
        let chunks = chunker::to_token_chunks(source_code.as_str()).into_iter().peekable();
        println!("chunks: {:?}", chunks.clone().collect::<Vec<String>>());
        let tokens = lexer::to_token_stream(chunks.collect());
        println!("tokens: {:?}", tokens);
        let ast = parse_to_program(tokens);
        println!("ast: {:?}", ast);
    }

  #[test]
  fn test_parse() {
    let tokens = vec![
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
    let ast = parse_to_program(tokens);
    let expected_ast = Program {
        items: vec![
            Item::ItemFn(ItemFn {
                signature: FnSignature {
                    ident: "main".to_string(),
                    args: vec![],
                    output: None,
                },
                block: vec![
                    Statement::Local(Local {
                        name: "x".to_string(),
                        var_type: "I32".to_string(),
                        value: Expr::ExprLit("10".to_string()),
                    }),
                ],
            }),
        ],
    };
    assert_eq!(format!("{:?}", ast), format!("{:?}", expected_ast));
  }
}
