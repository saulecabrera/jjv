//! Javy JavaScript Source Code Viewer.
//!
//! Inspects Javy generated WebAssembly modules and prints original JavaScript
//! source.
//!
//! TODO: Javy can generate uncompressed modules, therefore, we should avoid
//! defaulting to using brotli unconditionally.
//!
//! Usage
//!
//! ```rust
//! cargo run -- /path/to/module.wasm
//! ```
use anyhow::{bail, Result};
use brotli::BrotliDecompress;
use std::io::Cursor;
use wasmparser::{Parser, Payload::*};

static CUSTOM_SECTION: &'static str = "javy_source";

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = args[1].clone();
    let bytes = std::fs::read(&path)?;
    let parser = Parser::new(0);
    let mut found = false;

    for payload in parser.parse_all(&bytes) {
        match payload? {
            CustomSection(reader) => {
                if reader.name() == CUSTOM_SECTION {
                    let compressed: Vec<u8> = reader.data().into();
                    let mut cursor = Cursor::new(compressed);
                    BrotliDecompress(&mut cursor, &mut std::io::stdout())?;
                    found = true;
                }
            }
            _ => {}
        }
    }

    if !found {
        bail!("{CUSTOM_SECTION} not found");
    }

    Ok(())
}
