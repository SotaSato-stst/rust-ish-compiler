use super::rst::*;
use super::syscall::*;
use crate::ast::program::{Expr, Item, ItemFn, Program, Statement};

pub fn generate_code(program: &Program) -> AsmCode {
    let mut asm_code = AsmCode::new();
    for item in &program.items {
        match item {
            Item::ItemFn(item_fn) => {
                handle_fn(&mut asm_code, item_fn);
            }
            _ => { /* Handle other item types if necessary */ }
        }
    }
    asm_code
}

fn handle_fn(asm_code: &mut AsmCode, item_fn: &ItemFn) {
    let mut instructions = Vec::<Instruction>::new();
    let mut data_directives = Vec::<DataDirective>::new();
    item_fn.block.iter().for_each(|stmt| {
        match stmt {
            Statement::FnCall(fn_call) => {
                if fn_call.name == "println!" {
                    if let Some(arg) = fn_call.args.get(0) {
                        match arg {
                            Expr::ExprLit(lit) => {
                                instructions.push(Instruction::MOVE {
                                    dest: Rst::RAX.to_string(),
                                    src: SYSCALL::WRITE.to_string(),
                                });
                                instructions.push(Instruction::MOVE {
                                    dest: Rst::RDI.to_string(),
                                    src: "1".to_string(), // stdout
                                });
                                let msg = "msg";
                                let msg_len = "msg_len";
                                instructions.push(Instruction::LOAD {
                                    dest: Rst::RSI.to_string(),
                                    addr: msg.to_string(),
                                });
                                data_directives.push(DataDirective::DB {
                                    left: msg.to_string(),
                                    right: vec![lit.clone(), "0x0A".to_string()],
                                });
                                instructions.push(Instruction::MOVE {
                                    dest: Rst::RDX.to_string(),
                                    src: msg_len.to_string(),
                                });
                                data_directives.push(DataDirective::EQUE {
                                    left: msg_len.to_string(),
                                    right: vec!["$ - msg".to_string()],
                                });
                                instructions.push(Instruction::SYSCALL);
                                handle_exit(&mut instructions);
                            }
                            _ => { /* Handle other expression types if necessary */ }
                        }
                    }
                }
            }
            _ => { /* Handle other statement types if necessary */ }
        }
    });
    asm_code.text_sec.push(FnCode {
        label: convert_to_asm_fn_name(&item_fn.signature.ident),
        instructions,
    });
    asm_code.data_sec.extend(data_directives);
}

fn handle_exit(instructions: &mut Vec<Instruction>) {
    instructions.push(Instruction::MOVE {
        dest: Rst::RAX.to_string(),
        src: SYSCALL::EXIT.to_string(),
    });
    instructions.push(Instruction::XOR {
        src1: Rst::RDI.to_string(),
        src2: Rst::RDI.to_string(),
    });
    instructions.push(Instruction::SYSCALL);
}

#[derive(Debug)]
pub struct AsmCode {
    directives: Vec<String>,
    text_sec: Vec<FnCode>,
    data_sec: Vec<DataDirective>,
}

impl AsmCode {
    fn new() -> Self {
        AsmCode {
            directives: vec!["global _main".to_string(), "default rel".to_string()],
            text_sec: Vec::<FnCode>::new(),
            data_sec: Vec::<DataDirective>::new(),
        }
    }

    pub fn serialize(&self) -> Vec<String> {
        let mut asm_lines = Vec::<String>::new();
        asm_lines.extend(self.directives.clone());
        asm_lines.push("section .data".to_string());
        for data_dir in &self.data_sec {
            asm_lines.extend(data_dir.serialize());
        }
        asm_lines.push("section .text".to_string());
        for fn_code in &self.text_sec {
            asm_lines.extend(fn_code.serialize());
        }
        asm_lines
    }
}

#[derive(Debug)]
enum DataDirective {
    DB { left: String, right: Vec<String> },
    EQUE { left: String, right: Vec<String> },
}

impl Serialize for DataDirective {
    fn serialize(&self) -> Vec<String> {
        match self {
            DataDirective::DB { left, right } => {
                let mut s = Vec::<String>::new();
                for r in right {
                    if r.starts_with("0x") {
                        s.push(r.clone());
                        continue;
                    }
                    s.push(format!("\"{}\"", r));
                }
                vec![format!("    {} db {}", left, s.join(", "))]
            }
            DataDirective::EQUE { left, right } => {
                vec![format!("    {} equ {}", left, right.join(" "))]
            }
        }
    }
}

#[derive(Debug)]
struct FnCode {
    label: String,
    instructions: Vec<Instruction>,
}

impl Serialize for FnCode {
    fn serialize(&self) -> Vec<String> {
        let mut lines = Vec::<String>::new();
        lines.push(format!("{}:", self.label));
        for instr in &self.instructions {
            lines.extend(instr.serialize());
        }
        lines
    }
}

impl Serialize for AsmCode {
    fn serialize(&self) -> Vec<String> {
        self.serialize()
    }
}

trait Serialize {
    fn serialize(&self) -> Vec<String>;
}

#[derive(Debug)]
enum Instruction {
    MOVE { dest: String, src: String },
    LOAD { dest: String, addr: String },
    SYSCALL,
    XOR { src1: String, src2: String },
}

impl Serialize for Instruction {
    fn serialize(&self) -> Vec<String> {
        match self {
            Instruction::MOVE { dest, src } => vec![format!("    mov {}, {}", dest, src)],
            Instruction::LOAD { dest, addr } => vec![format!("    lea {}, [{}]", dest, addr)],
            Instruction::SYSCALL => vec!["    syscall".to_string()],
            Instruction::XOR { src1, src2 } => vec![format!("    xor {}, {}", src1, src2)],
        }
    }
}

fn convert_to_asm_fn_name(fn_name: &String) -> String {
    if fn_name == "main" {
        "_main".to_string()
    } else {
        fn_name.clone()
    }
}
