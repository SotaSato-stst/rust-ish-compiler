mod ast;
mod code_gen;
mod libs;
pub mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }
    let output_filename = "output.asm";

    let filename = &args[1];
    let source_code = libs::readfile(filename);
    let asm_code = compile_source(&source_code);
    output_asm_file(&asm_code, output_filename);
}

fn compile_source(source_code: &String) -> code_gen::code_gen::AsmCode {
    let program = parser::parser::parse(source_code);
    println!("Parsed AST: {:?}", program);
    let code = code_gen::code_gen::generate_code(&program);
    println!("Generated Assembly Code: {:?}", code.serialize());
    output_asm_file(&code, "./misc/output.asm");
    code
}

fn output_asm_file(asm_code: &code_gen::code_gen::AsmCode, output_filename: &str) {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(output_filename).expect("Could not create output file");
    let asm_string = asm_code.serialize();
    let mut writer = std::io::BufWriter::new(&mut file);
    for line in asm_string {
        writeln!(writer, "{}", line).expect("Could not write to output file");
    }
    writer.flush().expect("Could not flush output file");
}

#[cfg(test)]
pub mod tests {
    use crate::libs;

    use super::*;
    #[test]
    fn for_test() {
        let filename = "./src/parser/test/sample.txt";
        let source_code = libs::readfile(filename);
        let _code = compile_source(&source_code);
    }
}
