//! Combing Through the Haystack (https://rosalind.info/problems/subs/)
//!
//! # Problem
//!
//!     Given: Two DNA Strings s and t (each of length at most 1 kbp).
//!
//!     Return: All locations of t as a substring of s.
//!
//! # Sample Dataset
//!     GATATATGCATATACTT
//!     ATAT
//!
//! # Sample Output
//!     2 4 10
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "subs";

/// Return the subcommand for SUBS
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the subs workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let (dna_string, substring) = parse_input(&input)?;
        let positions = find_pos(dna_string, substring)?;
        print(&positions);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Print output
fn print(positions: &[usize]) {
    for pos in positions.iter() {
        print!("{} ", pos);
    }
    println!()
}

/// Parse input
fn parse_input(input: &'_ str) -> Result<(&'_ str, &'_ str)> {
    input
        .split_once('\n')
        .ok_or_else(|| Error::new(ErrorKind::IO, "invalid input"))
}

/// Find the positions of the substring
fn find_pos(dna_string: &str, substring: &str) -> Result<Vec<usize>> {
    if substring.len() > dna_string.len() {
        return Ok(Vec::new());
    }
    let mut positions = Vec::new();
    for (i, sub) in dna_string.as_bytes().windows(substring.len()).enumerate() {
        if String::from_utf8(sub.to_owned())? == substring {
            positions.push(i + 1);
        }
    }

    Ok(positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pos() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a str,
            substring: &'a str,
            expected: Result<Vec<usize>>,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset from problem",
            dna_string: "GATATATGCATATACTT",
            substring: "ATAT",
            expected: Ok(vec![2, 4, 10]),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                find_pos(test_case.dna_string, test_case.substring),
                "{}",
                test_case.name,
            );
        }
    }
}
