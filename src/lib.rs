use anyhow::Result;
use brotli::BrotliDecompress;
use std::io::Cursor;
use std::path::PathBuf;
use wasmparser::{Parser, Payload::*};

static CUSTOM_SECTION: &'static str = "javy_source";

pub fn extract(path: &PathBuf) -> Result<Option<Vec<u8>>> {
    let bytes = std::fs::read(&path)?;
    from_wasm_bytes(&bytes)
}

pub fn from_wasm_bytes(bytes: &[u8]) -> Result<Option<Vec<u8>>> {
    let parser = Parser::new(0);
    let mut result: Vec<u8> = vec![];

    for payload in parser.parse_all(&bytes) {
        match payload? {
            CustomSection(reader) => {
                if reader.name() == CUSTOM_SECTION {
                    let original: Vec<u8> = reader.data().into();

                    let mut cursor = Cursor::new(original.clone());
                    match BrotliDecompress(&mut cursor, &mut result) {
                        Ok(_) => {}
                        Err(_) => {
                            result = original;
                        }
                    }
                    break;
                }
            }
            _ => {}
        }
    }

    if result.len() > 0 {
        Ok(Some(result))
    } else {
        Ok(None)
    }
}
