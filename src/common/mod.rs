use crate::errors::{Error, ErrorKind, Result};
use std::io::Read;

pub mod dna;
pub mod fasta;

pub const FILE_ARG: &str = "file";

/// Load simple input
///
/// Load the entire file as a string and trim off leading and trailing white spaces.
///
pub fn load_simple(path: &str) -> Result<String> {
    let mut string = String::new();
    std::fs::File::open(path)?.read_to_string(&mut string)?;
    Ok(string.trim().into())
}

/// Return a generic clap command with a file input
///
/// The generated command looks like the following:
///     SUBCOMMAND <file>
/// where file is the path to the input txt from Rosalind.
///
pub fn subcommand_file(subcommand: &str) -> clap::Command<'static> {
    clap::Command::new(subcommand).arg(clap::Arg::new(FILE_ARG).required(true))
}

/// Return argument error
pub fn argument_err() -> Error {
    Error::new(ErrorKind::User, &format!("{} argument required", FILE_ARG))
}
