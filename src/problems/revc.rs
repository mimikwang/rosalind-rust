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
use crate::errors::{Error, ErrorKind, Result};

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
    dna_string
        .chars()
        .rev()
        .map(|base| {
            complement_base(base)
                .ok_or_else(|| Error::new(ErrorKind::IO, "invalid base found in dna string"))
        })
        .collect()
}

/// Complement a base
///
/// Returns the complement if a DNA base.  If the base is not a valid DNA base, None is returned.
fn complement_base(base: char) -> Option<char> {
    match base {
        'A' => Some('T'),
        'T' => Some('A'),
        'C' => Some('G'),
        'G' => Some('C'),
        _ => None,
    }
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
        let test_cases = [
            TestCase {
                name: "Sample Dataset from problem",
                dna_string: "AAAACCCGGT".into(),
                expected: Ok("ACCGGGTTTT".into()),
            },
            TestCase {
                name: "Should error out with invalid base",
                dna_string: "AAAZG".into(),
                expected: Err(Error::new(
                    ErrorKind::IO,
                    "invalid base found in dna string",
                )),
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                reverse_complement(test_case.dna_string),
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_complement_base() {
        struct TestCase<'a> {
            name: &'a str,
            base: char,
            expected: Option<char>,
        }
        let test_cases = [
            TestCase {
                name: "Should return 'A' for 'T'",
                base: 'T',
                expected: Some('A'),
            },
            TestCase {
                name: "Should return 'T' for 'A'",
                base: 'A',
                expected: Some('T'),
            },
            TestCase {
                name: "Should return 'C' for 'G'",
                base: 'G',
                expected: Some('C'),
            },
            TestCase {
                name: "Should return 'G' for 'C'",
                base: 'C',
                expected: Some('G'),
            },
            TestCase {
                name: "Should return None for invalid DNA base",
                base: 'Z',
                expected: None,
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                complement_base(test_case.base),
                "{}",
                test_case.name
            );
        }
    }
}
