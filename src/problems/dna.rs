//! Counting DNA Nucleotides (https://rosalind.info/problems/dna/)
//!
//! # Problem
//!
//!     Given: A DNA string s of length at most 1000 nt.
//!
//!     Return: Four integers (separated by spaces) counting the respective number of times that
//!             the symbols 'A', 'C', 'G', and 'T' occur in s.
//!
//! # Sample Dataset
//!     AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC
//!
//! # Sample Output
//!     20 12 17 21
//!
use crate::common;
use crate::errors::Result;
use std::collections::BTreeMap;

pub const SUBCOMMAND: &str = "dna";

/// Return subcommand for DNA
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the dna workflow
///
/// Run the workflow by loading the dna string from the input text file, building a counter,
/// formatting the output, and printing it.
///
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let dna_string = common::load_simple(path)?;
        let counter = count_bases(&dna_string);
        let output = format_output(counter);
        println!("{}", output);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Count bases in the dna string and return a BTreeMap
///
/// Any bases that are not 'A', 'C', 'G', or 'T' are ignored.
///
fn count_bases(dna_string: &str) -> BTreeMap<char, usize> {
    let mut counter = BTreeMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for base in dna_string.chars() {
        match counter.get_mut(&base) {
            Some(count) => {
                *count += 1;
            }
            _ => continue,
        }
    }
    counter
}

/// Format output
fn format_output(counter: BTreeMap<char, usize>) -> String {
    format!(
        "{} {} {} {}",
        counter.get(&'A').unwrap_or(&0),
        counter.get(&'C').unwrap_or(&0),
        counter.get(&'G').unwrap_or(&0),
        counter.get(&'T').unwrap_or(&0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bases() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a str,
            expected: BTreeMap<char, usize>,
        }
        let test_cases = [
            TestCase {
                name: "Sample Dataset from problem",
                dna_string:
                    "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC",
                expected: BTreeMap::from([('A', 20), ('C', 12), ('G', 17), ('T', 21)]),
            },
            TestCase {
                name: "Should ignore non DNA nucleotides",
                dna_string: "ZZZZXagct",
                expected: BTreeMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]),
            },
            TestCase {
                name: "Should not fail on empty string",
                dna_string: "",
                expected: BTreeMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]),
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                count_bases(test_case.dna_string),
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_format_output() {
        struct TestCase<'a> {
            name: &'a str,
            counter: BTreeMap<char, usize>,
            expected: String,
        }
        let test_cases = [
            TestCase {
                name: "Should format counter",
                counter: BTreeMap::from([('A', 123), ('C', 3), ('G', 42), ('T', 10)]),
                expected: "123 3 42 10".into(),
            },
            TestCase {
                name: "Should format an empty counter",
                counter: BTreeMap::new(),
                expected: "0 0 0 0".into(),
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                format_output(test_case.counter),
                "{}",
                test_case.name
            );
        }
    }
}
