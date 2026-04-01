use clap::Parser;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
enum GrssError {
    UnexpectedError,
    FileNotFound,
    PermissionDenied,
}

impl fmt::Display for GrssError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            GrssError::UnexpectedError => write!(f, "An unexpected error occurred!"),
            GrssError::FileNotFound => write!(f, "Could not find the file"),
            GrssError::PermissionDenied => write!(f, "Cannot open the file, permission denied."),
        }
    }
}

impl Error for GrssError {}

impl From<io::Error> for GrssError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => GrssError::FileNotFound,
            io::ErrorKind::PermissionDenied => GrssError::PermissionDenied,
            _ => GrssError::UnexpectedError,
        }
    }
}

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

fn run() -> Result<(), GrssError> {
    let args = Cli::parse();

    let file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(&args.path)?;

    let reader = BufReader::new(file);

    for (nu, read_result) in reader.lines().into_iter().enumerate() {
        let line = read_result.expect("An error occurred while reading the lines.");

        // Exact matching (case-sensitive)
        if line.contains(&args.pattern) {
            println!("{}| {}", nu + 1, line);
        }
    }

    Ok(())
}
