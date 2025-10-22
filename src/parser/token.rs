#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    RBrace,
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
    Literal(String),
    Const,
    Type(Type),
    Operator(Operator),
    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    I32,
    F64,
}
