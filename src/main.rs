use anyhow::Context;
use clap::Parser;
use std::fs::{File};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        println!("{error}");
    }
}

fn run() -> anyhow::Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let reader = BufReader::new(file);
    let mut output = BufWriter::new(io::stdout());

    for (nu, read_result) in reader.lines().enumerate() {
        let line = read_result.context("An error occurred while reading the lines.")?;

        // Exact matching (case-sensitive)
        if line.contains(&args.pattern) {
            writeln!(output, "{}| {}", nu + 1, line)?;
        }
    }

    output.flush()?;

    Ok(())
}
