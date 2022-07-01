//! Finding a Shared Motif (https://rosalind.info/problems/lcsm/)
//!
//! # Problem
//!
//!     Given: A collection of k (k <= 100) DNA strings of length at most 1 kbp each in FASTA
//!             format.
//!
//!     Return: A longest common substring of the collection.  (If multiple solutions exist, you
//!             may return any single solution.)
//!
//! # Sample Dataset
//!     >Rosalind_1
//!     GATTACA
//!     >Rosalind_2
//!     TAGACCA
//!     >Rosalind_3
//!     ATACA
//!
//! # Sample Output
//!     AC
//!
use crate::common;
use crate::errors::Result;
use std::collections::BTreeSet;

pub const SUBCOMMAND: &str = "lcsm";

/// Return subcommand for LCSM
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the lcsm workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let reader = common::fasta::Reader::new(std::fs::File::open(path)?);
        let substrings = find_substrings(reader)?;
        let longest = get_longest(substrings).unwrap_or_default();
        println!("{}", longest);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Get longest substring
fn get_longest(substrings: BTreeSet<String>) -> Option<String> {
    let mut sorted = Vec::from_iter(substrings);
    sorted.sort_by_key(|s| std::cmp::Reverse(s.len()));
    sorted.first().map(|substring| substring.to_owned())
}

/// Find all shared substrings
fn find_substrings<R>(reader: common::fasta::Reader<R>) -> Result<BTreeSet<String>>
where
    R: std::io::Read,
{
    let mut output = BTreeSet::new();
    for (i, result) in reader.iter().enumerate() {
        let record = result?;
        if i == 0 {
            output = chop_sequence(&record.sequence)?;
        } else {
            for substring in output.clone() {
                if !record.sequence.contains(&substring) {
                    output.remove(&substring);
                }
            }
        }
    }
    Ok(output)
}

/// Chop sequence into substrings
fn chop_sequence(seq: &str) -> Result<BTreeSet<String>> {
    let mut substrings = BTreeSet::new();
    for len in 2..=seq.len() {
        for chunk in seq.as_bytes().windows(len) {
            let substring = std::str::from_utf8(chunk)?.to_owned();
            substrings.insert(substring);
        }
    }
    Ok(substrings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chop_sequence() {
        struct TestCase<'a> {
            name: &'a str,
            seq: &'a str,
            expected: Result<BTreeSet<String>>,
        }
        let test_cases = [TestCase {
            name: "Should chop up sequences",
            seq: "AAAC",
            expected: Ok(BTreeSet::from([
                "AA".to_owned(),
                "AC".to_owned(),
                "AAA".to_owned(),
                "AAC".to_owned(),
                "AAAC".to_owned(),
            ])),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                chop_sequence(test_case.seq),
                "{}",
                test_case.name
            );
        }
    }
}
