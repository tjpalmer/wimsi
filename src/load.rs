use crate::util::*;
use std::convert::TryInto;

struct Module<'a> {
    data: &'a Vec<u8>,
}

impl<'a> Module<'a> {
    pub fn parse(&mut self) -> Try<()> {
        println!("{}", self.data.len());
        // Magic
        let expected_magic = [0, 'a' as u8, 's' as u8, 'm' as u8];
        if self.data[..4] != expected_magic {
            return Err(err("bad magic"));
        }
        // Version
        let version = u32::from_le_bytes(self.data[4..8].try_into()?);
        if version != 1 {
            return Err(err("bad version"));
        }
        // Done
        Ok(())
    }
}

pub fn run(data: &Vec<u8>) -> Try<()> {
    let mut module = Module { data };
    module.parse()?;
    Ok(())
}
