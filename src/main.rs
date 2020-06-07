use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use wasmparser::Parser;
use wasmparser::ParserState;
use wasmparser::WasmDecoder;

type Try<Value> = Result<Value, Box<dyn Error>>;

fn main() -> Try<()> {
    let ref buf: Vec<u8> = read_wasm_bytes()?;
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
                println!("  Export {} {:?}", get_name(field), kind);
            }
            ParserState::ImportSectionEntry { module, field, .. } => {
                println!("  Import {}::{}", get_name(module), get_name(field))
            }
            ParserState::EndWasm => break,
            _ => ( /* println!(" Other {:?}", state) */ ),
        }
    }
    Ok(())
}

fn get_name(bytes: &str) -> &str {
    // str::from_utf8(bytes).ok().unwrap()
    bytes
}

fn read_wasm_bytes() -> Try<Vec<u8>> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
