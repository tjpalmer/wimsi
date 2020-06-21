mod util;
use std::{fs::File, io::prelude::*};
use util::*;
use wasmparser::{Parser, ParserState, WasmDecoder};

fn main() -> Try<()> {
    let buf = read_bytes()?;
    if true {
        let mut parser = Parser::new(&buf);
        loop {
            let state = parser.read();
            if !handle_state(state) {
                break;
            }
        }
    }
    Ok(())
}

fn handle_state(state: &ParserState) -> bool {
    match state {
        ParserState::BeginFunctionBody { .. } => {
            println!("Begin function body");
        }
        ParserState::BeginWasm { .. } => {
            println!("====== Module");
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
