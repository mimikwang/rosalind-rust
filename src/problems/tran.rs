//! Transitions and Transversions (https://rosalind.info/problems/tran/)
//!
//! # Problem
//!
//!     Given: Two DNA strings s_1 and s_2 of equal length (at most 1kbp).
//!
//!     Return: The transition/transversion ratio R(s_1, s_2).
//!
//! # Sample Dataset
//!     >Rosalind_0209
//!     GCAACGCACAACGAAAACCCTTAGGGACTGGATTATTTCGTGATCGTTGTAGTTATTGGA
//!     AGTACGGGCATCAACCCAGTT
//!     >Rosalind_2200
//!     TTATCTGACAAAGAAAGCCGTCAACGGCTGGATAATTTCGCGATCGTGCTGGTTACTGGC
//!     GGTACGAGTGTTCCTTTGGGT
//!
//! # Sample Output
//!     1.21428571429
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "tran";

pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let mut reader = common::fasta::Reader::new(std::fs::File::open(path)?);
        let mut s_1 = common::fasta::Record::new();
        let mut s_2 = common::fasta::Record::new();
        reader.read(&mut s_1)?;
        reader.read(&mut s_2)?;
        let ratio = get_ratio(s_1.sequence.as_bytes(), s_2.sequence.as_bytes())?;
        println!("{}", ratio);
        return Ok(());
    }
    Err(common::argument_err())
}

fn get_ratio(s_1: &[u8], s_2: &[u8]) -> Result<f64> {
    let mut counter = Counter::new();
    counter.count(s_1, s_2)?;
    Ok(counter.ratio())
}

/// Counter counts transitions and transversions
#[derive(Debug, PartialEq)]
struct Counter {
    transition: usize,
    transversion: usize,
}

impl Counter {
    fn new() -> Self {
        Counter {
            transition: 0,
            transversion: 0,
        }
    }

    fn count(&mut self, s_1: &[u8], s_2: &[u8]) -> Result<()> {
        if s_1.len() != s_2.len() {
            return Err(Error::new(
                ErrorKind::User,
                "s_1 and s_2 must have the same length",
            ));
        }
        s_1.iter()
            .zip(s_2.iter())
            .for_each(|(b_1, b_2)| match classify(b_1, b_2) {
                Kind::Transition => self.transition += 1,
                Kind::Transversion => self.transversion += 1,
                _ => (),
            });
        Ok(())
    }

    fn ratio(&self) -> f64 {
        self.transition as f64 / self.transversion as f64
    }
}

#[derive(Debug, PartialEq)]
enum Kind {
    Transversion,
    Transition,
    Match,
    Unknown,
}

fn classify(b_1: &u8, b_2: &u8) -> Kind {
    if b_1 == b_2 {
        return Kind::Match;
    }
    match (b_1, b_2) {
        (b'A', b'G') | (b'G', b'A') | (b'C', b'T') | (b'T', b'C') => Kind::Transition,
        (b'A', b'C') | (b'A', b'T') => Kind::Transversion,
        (b'C', b'A') | (b'C', b'G') => Kind::Transversion,
        (b'T', b'A') | (b'T', b'G') => Kind::Transversion,
        (b'G', b'C') | (b'G', b'T') => Kind::Transversion,
        _ => Kind::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ratio() {
        let s_1 =
            b"GCAACGCACAACGAAAACCCTTAGGGACTGGATTATTTCGTGATCGTTGTAGTTATTGGAAGTACGGGCATCAACCCAGTT";
        let s_2 =
            b"TTATCTGACAAAGAAAGCCGTCAACGGCTGGATAATTTCGCGATCGTGCTGGTTACTGGCGGTACGAGTGTTCCTTTGGGT";
        let ratio = get_ratio(s_1, s_2);
        assert!(ratio.is_ok());

        let ratio = ratio.unwrap();
        assert!(ratio - 1.21428571429 < 0.01);
    }

    #[test]
    fn test_classify() {
        struct TestCase<'a> {
            name: &'a str,
            b_1: &'a u8,
            b_2: &'a u8,
            expected: Kind,
        }
        let test_cases = [
            TestCase {
                name: "Transition",
                b_1: &b'A',
                b_2: &b'G',
                expected: Kind::Transition,
            },
            TestCase {
                name: "Match",
                b_1: &b'C',
                b_2: &b'C',
                expected: Kind::Match,
            },
            TestCase {
                name: "Transversion",
                b_1: &b'T',
                b_2: &b'G',
                expected: Kind::Transversion,
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                classify(test_case.b_1, test_case.b_2),
                "{}",
                test_case.name,
            );
        }
    }

    #[test]
    fn test_counter_count() {
        struct TestCase<'a> {
            name: &'a str,
            s_1: &'a [u8],
            s_2: &'a [u8],
            expected: Result<()>,
            expected_counter: Counter,
        }
        let test_cases = [TestCase {
            name: "Example 1",
            s_1: b"AAGG",
            s_2: b"GAAT",
            expected: Ok(()),
            expected_counter: Counter {
                transition: 2,
                transversion: 1,
            },
        }];
        for test_case in test_cases {
            let mut counter = Counter::new();
            let actual = counter.count(test_case.s_1, test_case.s_2);
            assert_eq!(test_case.expected, actual, "{}", test_case.name,);
            assert_eq!(test_case.expected_counter, counter, "{}", test_case.name,);
        }
    }
}
