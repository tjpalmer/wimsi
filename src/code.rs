pub struct Computer {
    pub memory: Vec<u8>,
}

impl Computer {
    pub fn new() -> Computer {
        Computer { memory: vec![] }
    }

    pub fn push_i16(&mut self, i: i16) {
        println!("Push i16 {}", i);
        self.memory.extend_from_slice(&i.to_le_bytes());
    }

    pub fn push_i32(&mut self, i: i32) {
        println!("Push i32 {}", i);
        // TODO Worth aligning???
        self.memory.extend_from_slice(&i.to_le_bytes());
    }

    pub fn push_opcode(&mut self, opcode: Opcode) {
        println!("Push op {:?}", opcode);
        self.push_i16(opcode as i16);
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
#[repr(u16)]
pub enum Opcode {
    Call = 0x10,
    ConstI16 = 0x8041,
    ConstI32 = 0x41,
    Nop = 0x01,
    Return = 0x0F,
    Unreachable = 0x00,
}
