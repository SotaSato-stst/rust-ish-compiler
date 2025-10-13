mod lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    lexer::ast_builder::build_ast(filename);
}
