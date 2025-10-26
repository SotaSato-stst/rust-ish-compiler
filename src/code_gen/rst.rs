#[derive(Debug, Clone, Copy)]
pub enum Rst {
    RAX,
    RDX,
    RCX,
    RDI,
    RSI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Rst {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rst::RAX => "rax",
            Rst::RDX => "rdx",
            Rst::RCX => "rcx",
            Rst::RDI => "rdi",
            Rst::RSI => "rsi",
            Rst::R8 => "r8",
            Rst::R9 => "r9",
            Rst::R10 => "r10",
            Rst::R11 => "r11",
            Rst::R12 => "r12",
            Rst::R13 => "r13",
            Rst::R14 => "r14",
            Rst::R15 => "r15",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
// println!("{}", r.as_str()); // 出力: rax
