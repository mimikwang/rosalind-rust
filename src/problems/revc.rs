//! Complementing a Strand of DNA (https://rosalind.info/problems/revc/)
//!
//! # Problem
//!
//!     Given: A DNA string s of lenght at most 1000 bp.
//!
//!     Return: The reverse complement s^c of s.
//!
//! # Sample Dataset
//!     AAAACCCGGT
//!
//! # Sample Output
//!     ACCGGGTTTT
//!
use crate::common;
use crate::errors::Result;

pub const SUBCOMMAND: &str = "revc";

/// Return the command for REVC
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the revc workflow
///
/// Run the workflow by loading the dna string from the input text file, reverse complementing it,
/// and printing it.
///
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let dna_string = common::load_simple(path)?;
        let rc = reverse_complement(dna_string)?;
        println!("{}", rc);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Reverse Complement a DNA string
///
/// An error is returned if there is an invalid DNA base in the input dna_string.
///
fn reverse_complement(dna_string: String) -> Result<String> {
    let rc = common::dna::reverse_complement(dna_string.as_bytes())?;
    let rc = String::from_utf8(rc.to_vec())?;
    Ok(rc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_complement() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: String,
            expected: Result<String>,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset from problem",
            dna_string: "AAAACCCGGT".into(),
            expected: Ok("ACCGGGTTTT".into()),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                reverse_complement(test_case.dna_string),
                "{}",
                test_case.name
            );
        }
    }
}
