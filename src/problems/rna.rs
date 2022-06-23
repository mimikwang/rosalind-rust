//! Transcribing DNA into RNA (https://rosalind.info/problems/rna/)
//!
//! # Problem
//!
//!     Given: A DNA string t having length at most 1000 nt.
//!
//!     Return: The transcribed RNA string of t.
//!
//! # Sample Dataset
//!     GATGGAACTTGACTACGTAAATT
//!
//! # Sample Output
//!     GAUGGAACUUGACUACGUAAAUU
//!
use crate::common;
use crate::errors::Result;

pub const SUBCOMMAND: &str = "rna";

/// Return the subcommand for RNA
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the rna workflow
///
/// Run the workflow by loading the dna string from the input text file, transcribing it to rna,
/// and then printing it.
///
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let dna_string = common::load_simple(path)?;
        let rna_string = transcribe(dna_string);
        println!("{}", rna_string);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Transcribe DNA to RNA
///
/// Replace any 'T' with 'U'.  Everything else is left alone.
///
fn transcribe(dna_string: String) -> String {
    dna_string
        .chars()
        .map(|base| if base == 'T' { 'U' } else { base })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcribe() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: String,
            expected: String,
        }
        let test_cases = [
            TestCase {
                name: "Sample Dataset from problem",
                dna_string: "GATGGAACTTGACTACGTAAATT".into(),
                expected: "GAUGGAACUUGACUACGUAAAUU".into(),
            },
            TestCase {
                name: "Should not fail on empty string",
                dna_string: String::new(),
                expected: String::new(),
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                transcribe(test_case.dna_string),
                "{}",
                test_case.name,
            );
        }
    }
}
