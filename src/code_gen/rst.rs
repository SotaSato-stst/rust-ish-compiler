#[derive(Debug, Clone, Copy)] // 必要に応じてトレイトを導出
pub enum Rst {
    RAX,
    RDX,
    RDI,
    RSI,
}

impl Rst {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rst::RAX => "rax",
            Rst::RDX => "rdx",
            Rst::RDI => "rdi",
            Rst::RSI => "rsi",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
// println!("{}", r.as_str()); // 出力: rax
