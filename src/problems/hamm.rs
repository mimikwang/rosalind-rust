//! Counting Point Mutations (https://rosalind.info/problems/hamm/)
//!
//! # Problem
//!
//!     Given: Two DNA strings s and t of equal length (not exceeding 1 kbp).
//!
//!     Return: The Hamming distance d_H(s,t)
//!
//! # Sample Dataset
//!     GAGCCTACTAACGGGAT
//!     CATCGTAATGACGGCCT
//!
//! # Sample Output
//!     7
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "hamm";

/// Return the subcommand for HAMM
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the hamm workflow
///
/// Run the workflow by loading the sequences from the text file, calculating the hamming distance
/// between the sequences, and then printing it.
///
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let (seq1, seq2) = parse_input(input)?;
        let distance = hamming_distance(&seq1, &seq2)?;
        println!("{}", distance);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Parse input into the two sequences
fn parse_input(input: String) -> Result<(String, String)> {
    let input: Vec<&str> = input.split('\n').collect();
    if input.len() != 2 {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    Ok((input[0].into(), input[1].into()))
}

/// Calculate the hamming distance
fn hamming_distance(seq1: &str, seq2: &str) -> Result<i64> {
    if seq1.len() != seq2.len() {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    Ok(seq1
        .chars()
        .zip(seq2.chars())
        .map(|(base1, base2)| if base1 != base2 { 1 } else { 0 })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        struct TestCase<'a> {
            name: &'a str,
            seq1: &'a str,
            seq2: &'a str,
            expected: i64,
            expect_error: bool,
        }
        let test_cases = [
            TestCase {
                name: "Sample Dataset form problem",
                seq1: "GAGCCTACTAACGGGAT",
                seq2: "CATCGTAATGACGGCCT",
                expected: 7,
                expect_error: false,
            },
            TestCase {
                name: "Should return an error if sequences are not the same size",
                seq1: "AGG",
                seq2: "ACCC",
                expected: 0,
                expect_error: true,
            },
        ];
        for test_case in test_cases {
            let actual = hamming_distance(test_case.seq1, test_case.seq2);
            if test_case.expect_error {
                assert!(actual.is_err(), "{}", test_case.name);
            } else {
                assert_eq!(Ok(test_case.expected), actual, "{}", test_case.name);
            }
        }
    }
}
