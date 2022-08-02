//! Locating Restriction Sites (https://rosalind.info/problems/revp/)
//!
//! # Problem
//!     Given: A DNA string of length at most 1 kbp in FASTA format.
//!
//!     Return: The position and length of every reverse palindrome in the string having length
//!             between 4 and 12.  You may return these pairs in any order.
//!
//! # Sample Dataset
//!     > Rosalind_24
//!     TCAATGCATGCGGGTCTATATGCAT
//!
//! # Sample Output
//!     4 6
//!     5 4
//!     6 6
//!     7 4
//!     17 4
//!     18 4
//!     20 6
//!     21 4
//!
use crate::common;
use crate::errors::Result;

pub const SUBCOMMAND: &str = "revp";

/// Return subcommand for REVP
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the revp workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let file = std::fs::File::open(path)?;
        let mut reader = common::fasta::Reader::new(file);
        let mut record = common::fasta::Record::new();
        reader.read(&mut record)?;
        let locations = locate(&record.sequence)?;
        for location in locations {
            println!("{}", location);
        }
        return Ok(());
    }
    Err(common::argument_err())
}

/// Locate restriction sites
fn locate(dna_string: &str) -> Result<Vec<Output>> {
    let mut outputs = vec![];
    for size in 4..=12 {
        for (i, substring) in dna_string.as_bytes().windows(size).enumerate() {
            if substring == common::dna::reverse_complement(substring)? {
                outputs.push(Output::new(i + 1, size));
            }
        }
    }
    Ok(outputs)
}

#[derive(Debug, PartialEq)]
struct Output {
    position: usize,
    length: usize,
}

impl Output {
    fn new(position: usize, length: usize) -> Self {
        Self { position, length }
    }
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.position, self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locate() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a str,
            expected: Result<Vec<Output>>,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            dna_string: "TCAATGCATGCGGGTCTATATGCAT",
            expected: Ok(vec![
                Output {
                    position: 5,
                    length: 4,
                },
                Output {
                    position: 7,
                    length: 4,
                },
                Output {
                    position: 17,
                    length: 4,
                },
                Output {
                    position: 18,
                    length: 4,
                },
                Output {
                    position: 21,
                    length: 4,
                },
                Output {
                    position: 4,
                    length: 6,
                },
                Output {
                    position: 6,
                    length: 6,
                },
                Output {
                    position: 20,
                    length: 6,
                },
            ]),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                locate(test_case.dna_string),
                "{}",
                test_case.name
            );
        }
    }
}
