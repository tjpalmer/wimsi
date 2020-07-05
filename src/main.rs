mod code;
mod util;
use code::*;
use std::{fs::File, io::prelude::*};
use util::*;
use wasmparser::{Operator, Parser, ParserState, WasmDecoder};

fn main() -> Try<()> {
    let buf = read_bytes()?;
    let mut computer = Computer::new();
    if true {
        let mut parser = Parser::new(&buf);
        loop {
            let state = parser.read();
            if !handle_state(state, &mut computer) {
                break;
            }
        }
    }
    println!("{:x?}", computer.memory);
    Ok(())
}

fn handle_operator(operator: &Operator, computer: &mut Computer) {
    match operator {
        Operator::Call { function_index } => {
            computer.push_opcode(Opcode::Call);
        }
        Operator::I32Const { value } => {
            computer.push_opcode(Opcode::ConstI32);
            computer.push_i32(*value);
        }
        _ => {
            println!("Other op {:?}", operator);
        }
    }
}

fn handle_state(state: &ParserState, computer: &mut Computer) -> bool {
    match state {
        ParserState::BeginFunctionBody { .. } => {
            println!("Begin function body");
        }
        ParserState::BeginWasm { .. } => {
            println!("====== Module");
        }
        ParserState::CodeOperator(ref operator) => {
            handle_operator(operator, computer);
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

fn read_bytes() -> Try<Vec<u8>> {
    let args: Vec<String> = std::env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
