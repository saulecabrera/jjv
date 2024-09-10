//! Javy JavaScript Source Code Viewer.
//!
//! Inspects Javy generated WebAssembly modules and prints original JavaScript
//! source.
//!
//! Usage
//!
//! ```rust
//! cargo run -- /path/to/module.wasm
//! ```

use anyhow::Result;
use clap::Parser;
use jjv::extract;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "jjv",
    version,
    about = "Javy JavaScript Source Code Viewer",
    long_about = None
)]
pub struct Cli {
    #[arg(value_name = "INPUT", required = true)]
    pub path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    if let Ok(Some(contents)) = extract(&args.path) {
        println!("{}", String::from_utf8(contents)?);
    } else {
        println!("No JavaScript source found");
    }

    Ok(())
}
