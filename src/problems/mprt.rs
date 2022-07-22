//! Finding a Protein Motif (https://rosalind.info/problems/mprt/)
//!
//! # Problem
//!     Given: At most 15 UniProt Protein Database access IDs
//!
//!     Return: For each protein processing the N-glycoslation motif, output its given access ID
//!             followed by a list of locations in the protein string where the motif can be found.
//!
//! # Sample Dataset
//!     A2Z669
//!     B5ZC00
//!     P07204_TRBM_HUMAN
//!     P20840_SAG1_YEAST
//!
//! # Sample Output
//!     B5ZC00
//!     85 118 142 306 395
//!     P07204_TRBM_HUMAN
//!     47 115 116 382 409
//!     P20840_SAG1_YEAST
//!     79 109 135 248 306 348 364 402 485 501 614
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "mprt";
const DELIMITER: &str = "\n";
const DELIMITER_ID: &str = "_";

/// Return the subcommand for MPRT
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the mprt workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let ids = parse_input(&input);
        for id in ids {
            let record = get_fasta(get_id(id))?;
            let indices = find_motifs(&record.sequence);
            if !indices.is_empty() {
                println!("{}", id);
                println!("{}", format_indices(indices));
            }
        }
        return Ok(());
    }
    Err(common::argument_err())
}

/// Parse input
fn parse_input(input: &'_ str) -> Vec<&'_ str> {
    input.split(DELIMITER).collect()
}

/// Format indices
fn format_indices(indices: Vec<usize>) -> String {
    if indices.is_empty() {
        return "".to_owned();
    }
    let mut output = format!("{}", indices[0]);
    if indices.len() == 1 {
        return output;
    }
    for i in 1..indices.len() {
        output = format!("{} {}", output, indices[i]);
    }
    output
}

/// Get fasta record
fn get_fasta(id: &str) -> Result<common::fasta::Record> {
    let url = format!("http://www.uniprot.org/uniprot/{}.fasta", id);
    let resp = match reqwest::blocking::get(url) {
        Err(_) => Err(Error::new(ErrorKind::IO, "request fails")),
        Ok(resp) => Ok(resp),
    }?;
    let mut reader = common::fasta::Reader::new(resp);
    let mut record = common::fasta::Record::new();
    reader.read(&mut record)?;
    Ok(record)
}

/// Get ID
fn get_id(id: &'_ str) -> &'_ str {
    match id.split_once(DELIMITER_ID) {
        None => id,
        Some((first, _)) => first,
    }
}

/// Find the start indices of the glycosylation motif in a sequence
fn find_motifs(sequence: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let sequence_chars: Vec<char> = sequence.chars().collect();
    for (i, substring) in sequence_chars.windows(4).enumerate() {
        if is_motif(substring) {
            indices.push(i + 1);
        }
    }
    indices
}

/// Checks to see if the given string is a glycosylation motif
fn is_motif(substring: &[char]) -> bool {
    if substring.len() != 4 {
        return false;
    }
    if substring[0] != 'N'
        || substring[1] == 'P'
        || substring[3] == 'P'
        || (substring[2] != 'S' && substring[2] != 'T')
    {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_motifs() {
        struct TestCase<'a> {
            name: &'a str,
            sequence: &'a str,
            expected: Vec<usize>,
        }
        let test_cases = [
            TestCase {
                name: "B5ZC00",
                sequence: "MKNKFKTQEELVNHLKTVGFVFANSEIYNGLANAWDYGPLGVLLKNNLKNLWWKEFVTKQKDVVGLDSAIILNPLVWKASGHLDNFSDPLIDCKNCKARYRADKLIESFDENIHIAENSSNEEFAKVLNDYEISCPTCKQFNWTEIRHFNLMFKTYQGVIEDAKNVVYLRPETAQGIFVNFKNVQRSMRLHLPFGIAQIGKSFRNEITPGNFIFRTREFEQMEIEFFLKEESAYDIFDKYLNQIENWLVSACGLSLNNLRKHEHPKEELSHYSKKTIDFEYNFLHGFSELYGIAYRTNYDLSVHMNLSKKDLTYFDEQTKEKYVPHVIEPSVGVERLLYAILTEATFIEKLENDDERILMDLKYDLAPYKIAVMPLVNKLKDKAEEIYGKILDLNISATFDNSGSIGKRYRRQDAIGTIYCLTIDFDSLDDQQDPSFTIRERNSMAQKRIKLSELPLYLNQKAHEDFQRQCQK",
                expected: vec![85, 118, 142, 306, 395],
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                find_motifs(test_case.sequence),
                "{}",
                test_case.name,
            );
        }
    }

    #[test]
    fn test_is_motif() {
        struct TestCase<'a> {
            name: &'a str,
            substring: &'a [char],
            expected: bool,
        }
        let test_cases = [
            TestCase {
                name: "Should return true",
                substring: &['N', 'A', 'S', 'A'],
                expected: true,
            },
            TestCase {
                name: "Should return false",
                substring: &['N', 'A', 'S', 'A', 'A'],
                expected: false,
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                is_motif(test_case.substring),
                "{}",
                test_case.name,
            );
        }
    }
}
