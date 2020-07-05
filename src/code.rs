pub struct Computer {
    memory: Vec<u8>,
}

impl Computer {
    pub fn new() -> Computer {
        Computer { memory: vec![] }
    }
}

struct Memory {
    begin: i32,
    limit: i32,
    size: i32,
}

struct Module {
    code: i32,
    memories: Vec<i32>,
    stack: i32,
}

enum Opcode {
    Call = 0x10,
    ConstI32 = 0x41,
    Nop = 0x01,
    Return = 0x0F,
    Unreachable = 0x00,
}
