use std::error::Error;
use clap::Parser;
use std::{fmt, fs};
use std::fmt::Formatter;
use std::io;
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

    // TODO: Optimize the memory usage. Use `BufReader`
    let content = fs::read_to_string(&args.path)?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
