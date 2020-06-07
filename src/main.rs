use std::{fs::File, io::prelude::*};
use wasmparser::{Parser, ParserState, WasmDecoder};

type Try<Value> = Result<Value, Box<dyn std::error::Error>>;

fn main() -> Try<()> {
    let buf = read_wasm_bytes()?;
    let mut parser = Parser::new(&buf);
    loop {
        let state = parser.read();
        match *state {
            ParserState::BeginWasm { .. } => {
                println!("====== Module");
            }
            ParserState::ExportSectionEntry {
                field, ref kind, ..
            } => {
                println!("  Export {} {:?}", field, kind);
            }
            ParserState::ImportSectionEntry { module, field, .. } => {
                println!("  Import {}::{}", module, field)
            }
            ParserState::EndWasm => break,
            _ => ( /* println!(" Other {:?}", state) */ ),
        }
    }
    Ok(())
}

fn read_wasm_bytes() -> Try<Vec<u8>> {
    let args: Vec<String> = std::env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
