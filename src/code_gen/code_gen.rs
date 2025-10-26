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

struct RstManagerInFn {
    val_register_map: std::collections::HashMap<String, Rst>,
    rsts_for_return: Vec<Rst>,
    rsts_for_arguments: Vec<Rst>,
    rsts_for_general: Vec<Rst>,
}

impl RstManagerInFn {
    fn new(args: &Vec<crate::ast::program::FnParams>) -> Self {
        let mut val_register_map = std::collections::HashMap::<String, Rst>::new();
        for (i, arg) in args.iter().enumerate() {
            let rst = match i {
                0 => Rst::RDI,
                1 => Rst::RSI,
                2 => Rst::RDX,
                3 => Rst::RCX,
                4 => Rst::R8,
                5 => Rst::R9,
                _ => break,
            };
            val_register_map.insert(arg.name.clone(), rst);
        }
        RstManagerInFn {
            val_register_map: val_register_map,
            rsts_for_general: vec![Rst::R10, Rst::R11, Rst::R12, Rst::R13, Rst::R14, Rst::R15]
                .into_iter()
                .rev()
                .collect(),
            rsts_for_arguments: vec![Rst::RDI, Rst::RSI, Rst::RDX, Rst::RCX, Rst::R8, Rst::R9]
                .into_iter()
                .rev()
                .collect(),
            rsts_for_return: vec![Rst::RAX, Rst::RDX].into_iter().rev().collect(),
        }
    }

    fn get_rst_from_map(&self, name: String) -> Rst {
        *self.val_register_map.get(&name).unwrap()
    }

    fn pop_general_rsts(&mut self, name: String) -> Rst {
        let gst = self.rsts_for_general.pop().unwrap();
        self.val_register_map.insert(name, gst);
        gst
    }

    fn init_general_rsts(&mut self) {
        self.rsts_for_general = vec![Rst::R10, Rst::R11, Rst::R12, Rst::R13, Rst::R14, Rst::R15]
            .into_iter()
            .rev()
            .collect();
    }

    fn pop_argument_rsts(&mut self) -> Rst {
        self.rsts_for_arguments.pop().unwrap()
    }

    fn init_argument_rsts(&mut self) {
        self.rsts_for_arguments = vec![Rst::RDI, Rst::RSI, Rst::RDX, Rst::RCX, Rst::R8, Rst::R9]
            .into_iter()
            .rev()
            .collect();
    }

    fn pop_return_rsts(&mut self) -> Rst {
        self.rsts_for_return.pop().unwrap()
    }

    fn init_return_rsts(&mut self) {
        self.rsts_for_return = vec![Rst::RAX, Rst::RDX].into_iter().rev().collect();
    }
}

fn handle_fn(asm_code: &mut AsmCode, item_fn: &ItemFn) {
    let mut instructions = Vec::<Instruction>::new();
    let mut data_directives = Vec::<DataDirective>::new();
    let mut rgt_manager = RstManagerInFn::new(item_fn.signature.args.as_ref());
    item_fn.block.iter().for_each(|stmt| {
        match stmt {
            Statement::FnCall(fn_call) => {
                handle_fn_call(
                    &mut instructions,
                    fn_call,
                    &mut rgt_manager,
                    &mut data_directives,
                );
            }
            Statement::Local(local) => {
                let rst = handle_experession(&mut instructions, &local.value, &mut rgt_manager);
                rgt_manager.val_register_map.insert(local.name.clone(), rst);
            }
            Statement::Return(ret) => {
                let ret_rst = handle_experession(&mut instructions, ret, &mut rgt_manager);
                instructions.push(Instruction::MOVE {
                    dest: rgt_manager.pop_return_rsts().to_string(),
                    src: ret_rst.to_string(),
                });
                instructions.push(Instruction::RET);
                rgt_manager.init_return_rsts();
            }
            _ => { /* Handle other statement types if necessary */ }
        }
    });
    if item_fn.signature.ident == "main" {
        handle_exit(&mut instructions);
    }
    asm_code.text_sec.push(FnCode {
        label: convert_to_asm_fn_name(&item_fn.signature.ident),
        instructions,
    });
    asm_code.data_sec.extend(data_directives);
}

fn handle_experession(
    instructions: &mut Vec<Instruction>,
    expr: &Expr,
    rst_manager: &mut RstManagerInFn,
) -> Rst {
    match expr {
        Expr::ExprLit(lit) => {
            let rst = rst_manager.pop_general_rsts("tmp".to_string());
            instructions.push(Instruction::MOVE {
                dest: rst.to_string(),
                src: lit.clone(),
            });
            rst
        }
        Expr::ExprVariable(var_name) => {
            let rst = rst_manager.get_rst_from_map(var_name.clone());
            rst
        }
        Expr::ExprFnCall(fn_call) => {
            handle_fn_call(instructions, fn_call, rst_manager, &mut Vec::new());
            let rst = rst_manager.pop_general_rsts(fn_call.name.clone());
            instructions.push(Instruction::MOVE {
                dest: rst.to_string(),
                src: rst_manager.pop_return_rsts().to_string(),
            });
            rst_manager.init_return_rsts();
            rst
        }
        Expr::ExprBinaryOp { left, op, right } => {
            let left_rst = handle_experession(instructions, left, rst_manager);
            let right_rst = handle_experession(instructions, right, rst_manager);
            match op {
                crate::ast::program::Operator::Plus => {
                    instructions.push(Instruction::ADD {
                        dest: left_rst.to_string(),
                        src: right_rst.to_string(),
                    });
                    left_rst
                }
                _ => {
                    // Handle other operators if necessary
                    unimplemented!()
                }
            }
        }
        _ => {
            // Handle other expression types if necessary
            unimplemented!()
        }
    }
}

fn handle_fn_call(
    instructions: &mut Vec<Instruction>,
    fn_call: &crate::ast::program::FnCall,
    rst_manager: &mut RstManagerInFn,
    data_directives: &mut Vec<DataDirective>,
) {
    if fn_call.name == "println!" {
        handle_println(instructions, fn_call, data_directives)
    }
    for arg in fn_call.args.iter() {
        match arg {
            Expr::ExprLit(lit) => {
                instructions.push(Instruction::MOVE {
                    dest: rst_manager.pop_argument_rsts().to_string(),
                    src: lit.clone(),
                });
            }
            Expr::ExprVariable(var_name) => {
                let rst = rst_manager
                    .val_register_map
                    .get(var_name)
                    .unwrap()
                    .to_string();
                instructions.push(Instruction::MOVE {
                    dest: rst_manager.pop_argument_rsts().to_string(),
                    src: rst,
                });
            }
            _ => {
                // Handle other argument expression types if necessary
                unimplemented!()
            }
        };
    }
    rst_manager.init_argument_rsts();
    instructions.push(Instruction::CALL {
        func: convert_to_asm_fn_name(&fn_call.name),
    });
}

fn handle_println(
    instructions: &mut Vec<Instruction>,
    fn_call: &crate::ast::program::FnCall,
    data_directives: &mut Vec<DataDirective>,
) {
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
            }
            _ => { /* Handle other expression types if necessary */ }
        }
    }
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
    RET,
    CALL { func: String },
    MOVE { dest: String, src: String },
    ADD { dest: String, src: String },
    LOAD { dest: String, addr: String },
    SYSCALL,
    XOR { src1: String, src2: String },
}

impl Serialize for Instruction {
    fn serialize(&self) -> Vec<String> {
        match self {
            Instruction::RET => vec!["    ret".to_string()],
            Instruction::CALL { func } => vec![format!("    call {}", func)],
            Instruction::MOVE { dest, src } => vec![format!("    mov {}, {}", dest, src)],
            Instruction::ADD { dest, src } => vec![format!("    add {}, {}", dest, src)],
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
