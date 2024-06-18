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

use anyhow::Result;
use jjv::extract;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = args[1].clone();
    if let Ok(Some(contents)) = extract(&path.into()) {
        println!("{}", String::from_utf8(contents)?);
    } else {
        println!("No JavaScript source found");
    }

    Ok(())
}
