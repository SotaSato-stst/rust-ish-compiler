mod parser;
mod libs;
mod ast;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let source_code = libs::readfile(filename);
    parser::parser::parse(&source_code);
}

#[cfg(test)]
pub mod tests {
    use crate::libs;

    use super::*;
    #[test]
    fn for_test() {
        let filename = "./src/parser/test/sample.txt";
        let source_code = libs::readfile(filename);
        let ast = parser::parser::parse(&source_code);
        println!("ast: {:?}", ast);
    }
}