pub struct Computer {
    pub memory: Vec<u8>,
}

impl Computer {
    pub fn new() -> Computer {
        Computer { memory: vec![] }
    }

    pub fn push_i32(&mut self, i: i32) {
        println!("Push i32 {}", i);
        self.memory.extend_from_slice(&i.to_le_bytes());
    }

    pub fn push_opcode(&mut self, opcode: Opcode) {
        println!("Push op {:?}", opcode);
        let val = opcode as u32;
        self.memory.extend_from_slice(&val.to_le_bytes());
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

#[derive(Debug)]
pub enum Opcode {
    Call = 0x10,
    ConstI32 = 0x41,
    Nop = 0x01,
    Return = 0x0F,
    Unreachable = 0x00,
}
