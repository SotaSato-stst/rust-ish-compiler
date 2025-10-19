use core::str;

use super::tokenizer;
use super::lexer;
use super::parser;

pub fn build_ast(source_text: &str) {
  let chunks = tokenizer::to_token_chunks(source_text).into_iter().peekable();
  let tokens = lexer::to_token_stream(chunks.collect());
  let ast = parser::parse(tokens);
  println!("{:?}", ast);
}

#[cfg(test)]
pub mod tests {
    use crate::libs;

    use super::*;
    #[test]
    fn for_test() {
        let filename = "./src/lexer/test/sample.txt";
        let source_code = libs::readfile(filename);
        build_ast(source_code.as_str());
    }
}
