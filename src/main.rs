use clap::Parser;
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    path: PathBuf
}

fn main() {
    let args = Cli::parse();

    println!("Pattern: {:?}, path: {:?}", args.pattern, args.path);
}
