/// cargo run -- clap Cargo.toml
use clap::Parser;
use anyhow::{Context, Result};

/// Search for parttern in a file and displays the lines that contains it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for 
    pattern: String,
    /// The path of the file to read
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    // println!("pattern: {pattern:?}\npath: {path:?}");

    //println!("cli {cli:?}");
    let mut i:i32 = 0;
    for line in content.lines() {
        i += 1;
        if line.contains(&args.pattern) {
            println!("{} : {}", i, line);
        }
    }
    Ok(())
}