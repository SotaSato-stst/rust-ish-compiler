pub enum SYSCALL {
    WRITE = 0x2000004,
    EXIT = 0x2000001,
}

impl SYSCALL {
    pub fn to_string(&self) -> String {
        match self {
            SYSCALL::WRITE => "0x2000004".to_string(),
            SYSCALL::EXIT => "0x2000001".to_string(),
        }
    }
}