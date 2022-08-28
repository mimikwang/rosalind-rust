//! RNA Splicing (https://rosalind.info/problems/splc/)
//!
//! # Problem
//!     Given: A DNA string s (of length at most 1 kbp) and a collectin of substrings s acting as
//!             introns.  All strings are given in FASTA format.
//!
//!     Return: A protein string resulting from transcribing and traslating the exons of s. (Note:
//!             Only one solution will exist for the dataset provided).
//!
//! # Sample Dataset
//!     >Rosalind_10
//!     ATGGTCTACATAGCTGACAAACAGCACGTAGCAATCGGTCGAATCTCGAGAGGCATATGGTCACATGATCGGTCGAGCGTGTTTCAAAGTTTGCGCCTAG
//!     >Rosalind_12
//!     ATCGGTCGAA
//!     >Rosalind_15
//!     ATCGGTCGAGCGTGT
//!
//! # Sample Output
//!     MVYIADKQHVASREAYGHMFKVCA
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "splc";

/// Return the subcommand for SPLC
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the splc workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let file = std::fs::File::open(path)?;
        let reader = common::fasta::Reader::new(file);
        let mut record_iter = reader.iter();
        let dna_string = record_iter
            .next()
            .ok_or_else(|| Error::new(ErrorKind::IO, "empty fasta file"))??
            .sequence;
        let mut substrings = vec![];
        for record in record_iter {
            let record = record?;
            substrings.push(record.sequence);
        }
        let mut references: Vec<&str> = vec![];
        for substring in substrings.iter() {
            references.push(substring);
        }
        let output = splice_and_translate(&dna_string, &references)?;
        println!("{}", output);
        return Ok(());
    }
    Err(common::argument_err())
}

fn splice_and_translate(dna_string: &str, substrings: &[&str]) -> Result<String> {
    let spliced = splice(dna_string, substrings);
    let mut translated = vec![];
    for chunk in spliced.as_bytes().chunks_exact(3) {
        if let Some(base) = common::dna::dna_to_protein(chunk)? {
            translated.push(base);
        } else {
            break;
        }
    }
    let translated = String::from_utf8(translated.to_vec())?;
    Ok(translated)
}

fn splice(dna_string: &str, substrings: &[&str]) -> String {
    let mut output = dna_string.to_owned();
    substrings
        .iter()
        .for_each(|substring| output = output.replace(substring, ""));
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splice_and_translate() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a str,
            substrings: &'a [&'a str],
            expected: Result<String>,
        }
        let test_cases = [
            TestCase {
                name: "Sample Dataset",
                dna_string: "ATGGTCTACATAGCTGACAAACAGCACGTAGCAATCGGTCGAATCTCGAGAGGCATATGGTCACATGATCGGTCGAGCGTGTTTCAAAGTTTGCGCCTAG",
                substrings: &["ATCGGTCGAA", "ATCGGTCGAGCGTGT"],
                expected: Ok("MVYIADKQHVASREAYGHMFKVCA".to_owned()),
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                splice_and_translate(test_case.dna_string, test_case.substrings),
                "{}",
                test_case.name,
            );
        }
    }

    #[test]
    fn test_splice() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a str,
            substrings: &'a [&'a str],
            expected: String,
        }
        let test_cases = [TestCase {
            name: "Should remove all substrings",
            dna_string: "ABCABCDEF",
            substrings: &["AB", "CD"],
            expected: "CEF".to_owned(),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                splice(test_case.dna_string, test_case.substrings),
                "{}",
                test_case.name,
            );
        }
    }
}
