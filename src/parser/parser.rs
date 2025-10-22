use super::chunker;
use super::lexer;
use super::token::Token;
use crate::{
    ast::program::{Expr, FnCall, FnParams, FnSignature, Item, ItemFn, Local, Program, Statement},
    parser::token::Operator,
};

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
    Program { items }
}

fn parse_item(token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>) -> Item {
    match token_iter.next().unwrap() {
        Token::Fn => return parse_item_fn(token_iter),
        _ => panic!("Unexpected token"),
    }
}

fn parse_item_fn(token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>) -> Item {
    let signature = parse_fn_signature(token_iter);
    let block = parse_block(token_iter);
    Item::ItemFn(ItemFn { signature, block })
}

fn parse_block(
    token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>,
) -> Vec<Statement> {
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
            }
            _ => statements.push(parse_statement(token_iter)),
        }
    }
    statements
}

fn parse_statement(
    token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>,
) -> Statement {
    match token_iter.next().unwrap() {
        Token::Let => parse_let_statement(token_iter),
        Token::Identifier(indent) => {
            let _indent = indent.clone();
            match token_iter.peek().unwrap() {
                Token::LParentheses => {
                    let fn_call: FnCall = parse_fn_call(token_iter, &_indent);
                    match token_iter.next().unwrap() {
                        Token::Semicolon => (),
                        other => panic!("Expected ';' found: {:?}", other),
                    };
                    Statement::FnCall(fn_call)
                }
                _ => panic!("Unexpected token: {:?}", token_iter.peek().unwrap()),
            }
        }
        Token::Return => {
            let expr = parse_expr(token_iter);
            match token_iter.next().unwrap() {
                Token::Semicolon => (),
                other => panic!("Expected ';' found: {:?}", other),
            };
            Statement::Return(expr)
        }
        other => panic!("Unexpected token in statement found: {:?}", other),
    }
}

fn parse_fn_call(
    token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>,
    ident: &String,
) -> FnCall {
    let args = parse_fn_arg(token_iter);
    FnCall {
        name: ident.clone(),
        args: args,
    }
}

fn parse_fn_arg(token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>) -> Vec<Expr> {
    match token_iter.next().unwrap() {
        Token::LParentheses => (),
        _ => panic!("Expected '(': {:?}", token_iter.peek().unwrap()),
    };
    let mut args = Vec::<Expr>::new();
    while token_iter.peek() != None {
        match token_iter.next().unwrap() {
            Token::RParentheses => {
                break;
            }
            Token::Identifier(lit) => {
                args.push(Expr::ExprVariable(lit.clone()));
                match token_iter.peek().unwrap() {
                    Token::Comma => (token_iter.next(),),
                    Token::RParentheses => continue,
                    other => panic!("Expected ',' or ) after argument: {:?}", other),
                };
            }
            Token::Literal(lit) => {
                args.push(Expr::ExprLit(lit.clone()));
                match token_iter.peek().unwrap() {
                    Token::Comma => (token_iter.next(),),
                    Token::RParentheses => continue,
                    other => panic!("Expected ',' or ) after argument: {:?}", other),
                };
            }
            _ => panic!(
                "Unexpected token in function arguments: {:?}",
                token_iter.peek().unwrap()
            ),
        }
    }
    args
}

fn parse_let_statement(
    token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>,
) -> Statement {
    let name = match token_iter.next().unwrap() {
        Token::Identifier(name) => name,
        _ => panic!("Expected identifier"),
    };
    let mut var_type = String::new();

    if token_iter.peek().unwrap() == &Token::Collon {
        token_iter.next();
        var_type = match token_iter.next().unwrap() {
            Token::Type(t) => format!("{:?}", t),
            _ => panic!("Expected type"),
        };
    }
    match token_iter.next().unwrap() {
        Token::Identifier(op) if op == "=" => (),
        _ => panic!("Expected '='"),
    };
    let expr = parse_expr(token_iter);

    match token_iter.next().unwrap() {
        Token::Semicolon => (),
        _ => panic!("Expected ';'"),
    };
    Statement::Local(Local {
        name,
        var_type,
        value: expr,
    })
}

fn parse_expr(token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>) -> Expr {
    let _next_token = token_iter.next().unwrap();
    let _next_next_token = token_iter.peek().unwrap().clone();
    match _next_next_token {
        Token::Operator(_) => {
            let left = match _next_token {
                Token::Identifier(lit) => Expr::ExprVariable(lit),
                Token::Literal(lit) => Expr::ExprLit(lit),
                _ => panic!("Expected literal"),
            };
            let op = match token_iter.next().unwrap() {
                Token::Operator(op) => op,
                _ => panic!("Expected operator: {:?} found", token_iter.peek().unwrap()),
            };
            let right = match token_iter.next().unwrap() {
                Token::Identifier(lit) => Expr::ExprVariable(lit),
                Token::Literal(lit) => Expr::ExprLit(lit),
                _ => panic!("Expected literal"),
            };
            return Expr::ExprBinaryOp {
                left: Box::new(left),
                op: op,
                right: Box::new(right),
            };
        }
        Token::Semicolon => match _next_token {
            Token::Identifier(lit) => return Expr::ExprVariable(lit),
            Token::Literal(lit) => return Expr::ExprLit(lit),
            _ => panic!("Expected literal"),
        },
        Token::LParentheses => match _next_token {
            Token::Identifier(ident) => {
                return Expr::ExprFnCall(parse_fn_call(token_iter, &ident));
            }
            _ => panic!("Expected function call"),
        },
        _ => panic!("Unexpected token in expression: {:?}", _next_next_token),
    }
}

fn parse_fn_signature(
    token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>,
) -> FnSignature {
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
            }
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

fn parse_fn_params(token_iter: &mut core::iter::Peekable<impl Iterator<Item = Token>>) -> FnParams {
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
    FnParams { name, arg_type }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libs;
    use crate::parser::token::{Token, Type};

    #[test]
    fn for_test() {
        let filename = "./src/parser/test/sample.txt";
        let source_code = libs::readfile(filename);
        let chunks = chunker::to_token_chunks(source_code.as_str())
            .into_iter()
            .peekable();
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
            items: vec![Item::ItemFn(ItemFn {
                signature: FnSignature {
                    ident: "main".to_string(),
                    args: vec![],
                    output: None,
                },
                block: vec![Statement::Local(Local {
                    name: "x".to_string(),
                    var_type: "I32".to_string(),
                    value: Expr::ExprLit("10".to_string()),
                })],
            })],
        };
        assert_eq!(format!("{:?}", ast), format!("{:?}", expected_ast));
    }

    #[test]
    fn test_parse_exp() {
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
            Token::Operator(Operator::Plus),
            Token::Identifier("20".to_string()),
            Token::Semicolon,
            Token::RBrace,
        ];
        let ast = parse_to_program(tokens);
        let expected_ast = Program {
            items: vec![Item::ItemFn(ItemFn {
                signature: FnSignature {
                    ident: "main".to_string(),
                    args: vec![],
                    output: None,
                },
                block: vec![Statement::Local(Local {
                    name: "x".to_string(),
                    var_type: "I32".to_string(),
                    value: Expr::ExprBinaryOp {
                        left: Box::new(Expr::ExprLit("10".to_string())),
                        op: Operator::Plus,
                        right: Box::new(Expr::ExprLit("20".to_string())),
                    },
                })],
            })],
        };
        assert_eq!(format!("{:?}", ast), format!("{:?}", expected_ast));
    }

    #[test]
    fn test_parse_function_call() {
        let tokens = vec![
            Token::Fn,
            Token::Identifier("main".to_string()),
            Token::LParentheses,
            Token::RParentheses,
            Token::LBrace,
            // let result = sum(1, 2);
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Identifier("=".to_string()), // 代入演算子をIdentifierやOperatorで表現
            Token::Identifier("sum".to_string()),
            Token::LParentheses,
            Token::Literal("1".to_string()), // 数値リテラル
            Token::Comma,
            Token::Literal("2".to_string()), // 数値リテラル
            Token::RParentheses,
            Token::Semicolon,
            Token::RBrace,
            // fn sum(int1: i32, int2: i32) -> i32 { ... }
            Token::Fn,
            Token::Identifier("sum".to_string()),
            Token::LParentheses,
            // int1: i32
            Token::Identifier("int1".to_string()),
            Token::Collon,
            Token::Type(Type::I32),
            Token::Comma,
            // int2: i32
            Token::Identifier("int2".to_string()),
            Token::Collon,
            Token::Type(Type::I32),
            Token::RParentheses,
            Token::Collon,          // 戻り値の型注釈前の ':'
            Token::Type(Type::I32), // 戻り値の型
            Token::LBrace,
            // let result = int1 + int2;
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Identifier("=".to_string()),
            Token::Identifier("int1".to_string()),
            Token::Operator(Operator::Plus),
            Token::Identifier("int2".to_string()),
            Token::Semicolon,
            // return result;
            Token::Return,
            Token::Identifier("result".to_string()),
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
                    block: vec![Statement::Local(Local {
                        name: "result".to_string(),
                        var_type: "".to_string(),
                        value: Expr::ExprFnCall(FnCall {
                            name: "sum".to_string(),
                            args: vec![
                                Expr::ExprLit("1".to_string()),
                                Expr::ExprLit("2".to_string()),
                            ],
                        }),
                    })],
                }),
                Item::ItemFn(ItemFn {
                    signature: FnSignature {
                        ident: "sum".to_string(),
                        args: vec![
                            FnParams {
                                name: "int1".to_string(),
                                arg_type: "I32".to_string(),
                            },
                            FnParams {
                                name: "int2".to_string(),
                                arg_type: "I32".to_string(),
                            },
                        ],
                        output: Some("I32".to_string()),
                    },
                    block: vec![
                        Statement::Local(Local {
                            name: "result".to_string(),
                            var_type: "".to_string(),
                            value: Expr::ExprBinaryOp {
                                left: Box::new(Expr::ExprVariable("int1".to_string())),
                                op: Operator::Plus,
                                right: Box::new(Expr::ExprVariable("int2".to_string())),
                            },
                        }),
                        Statement::Return(Expr::ExprVariable("result".to_string())),
                    ],
                }),
            ],
        };
        assert_eq!(format!("{:?}", ast), format!("{:?}", expected_ast));
    }
}
