use anyhow::Context;
use clap::Parser;
use log::{debug, error};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Stdout, Write, stdout};
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
    env_logger::init();
    if let Err(error) = run() {
        error!("{}", error)
    }
}

fn run() -> anyhow::Result<()> {
    let args = Cli::parse();

    debug!("Opening file `{:?}`", &args.path);
    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let reader = BufReader::new(file);
    let mut output = BufWriter::new(stdout());

    read_file(reader, &args.pattern, &mut output)?;

    output.flush()?;

    Ok(())
}

/// Consumes the given `BufReader<R>` until EOF is found.
///
/// If the pattern is found, append the line to the
/// buffered output.
///
/// This function does not flush automatically the output at the end
/// but, can eagerly flush if the buffer is full.
fn read_file(
    mut reader: BufReader<File>,
    pattern: &str,
    output: &mut BufWriter<Stdout>,
) -> anyhow::Result<()> {
    let mut line = String::new();
    let mut line_number = 1;

    loop {
        line.clear();

        let bytes_read = reader
            .read_line(&mut line)
            .context("An error occurred while reading the lines.")?;

        if bytes_read == 0 {
            debug!("Reached EOF (line {})", line_number);
            break; // Reach EOF
        }

        // Exact matching (case-sensitive)
        if line.contains(pattern) {
            let has_line_separator = line.ends_with('\n');

            if has_line_separator {
                write!(output, "{}| {}", line_number, line)?;
            } else {
                writeln!(output, "{}| {}", line_number, line)?;
            }
        }
        line_number += 1;
    }

    Ok(())
}
