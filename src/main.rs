mod code;
mod util;
use code::*;
use std::{fs::File, io::prelude::*};
use util::*;
use wasmparser::{Operator, Parser, ParserState, WasmDecoder};

fn main() -> Try<()> {
    let buf = read_bytes()?;
    let mut computer = Computer::new();
    let mut coder = Coder { computer: &mut computer };
    if true {
        let mut parser = Parser::new(&buf);
        loop {
            let state = parser.read();
            if !coder.handle_state(state) {
                break;
            }
        }
    }
    println!("{:x?}", computer.memory);
    Ok(())
}

struct Coder<'a> {

    computer: &'a mut Computer,

}

impl<'a> Coder<'a> {

    fn handle_operator(&mut self, operator: &Operator) {
        let computer = &mut self.computer;
        match operator {
            Operator::Call { function_index } => {
                computer.push_opcode(Opcode::Call);
            }
            Operator::I32Const { value } => {
                if *value as i16 as i32 == *value {
                    computer.push_opcode(Opcode::ConstI16);
                    computer.push_i16(*value as i16);
                } else {
                    computer.push_opcode(Opcode::ConstI32);
                    computer.push_i32(*value);
                }
            }
            _ => {
                println!("Other op {:?}", operator);
            }
        }
    }

    fn handle_state(&mut self, state: &ParserState) -> bool {
        match state {
            ParserState::BeginFunctionBody { .. } => {
                println!("Begin function body");
            }
            ParserState::BeginWasm { .. } => {
                println!("====== Module");
            }
            ParserState::CodeOperator(ref operator) => {
                self.handle_operator(operator);
            }
            ParserState::EndFunctionBody => {
                println!("End function body");
            }
            ParserState::EndWasm => {
                return false;
            }
            ParserState::ExportSectionEntry {
                field,
                ref kind,
                index,
            } => {
                println!("Export {} {:?} at {}", field, kind, index);
            }
            ParserState::ImportSectionEntry { module, field, ty } => {
                // wasmparser::ImportSectionEntryType
                println!("Import {}::{} of {:?}", module, field, ty);
            }
            ParserState::TypeSectionEntry(ref func_type) => {
                // wasmparser::FuncType
                println!("Type section entry: {:?}", func_type);
            }
            _ => {
                println!("Other {:?}", state);
            }
        }
        true
    }

}

fn read_bytes() -> Try<Vec<u8>> {
    let args: Vec<String> = std::env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
