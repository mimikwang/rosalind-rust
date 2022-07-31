//! Open Reading Frames (https://rosalind.info/problems/orf/)
//!
//! # Problem
//!     Given: A DNA string s of length at most 1 kbp in FASTA format.
//!
//!     Return: Every distinct candidate protein string that can be translated from ORFs of s.
//!             Strings can be returned in any order.
//!
//! # Sample Dataset
//!     >Rosalind_99
//!     AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG
//!
//! # Sample Output
//!     MLLGSFRLIPKETLIQVAGSSPCNLS
//!     M
//!     MGMTPRLGLESLLE
//!     MTPRLGLESLLE
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};
use std::collections::BTreeSet;

pub const SUBCOMMAND: &str = "orf";
const START_CODON: u8 = b'M';
const END_CODON: u8 = b'-';

/// Return the subcommand for ORF
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the orf workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let file = std::fs::File::open(path)?;
        let mut reader = common::fasta::Reader::new(file);
        let mut record = common::fasta::Record::new();
        reader.read(&mut record)?;
        let output = run_workflow(&record.sequence)?;
        for protein in output {
            println!("{}", protein);
        }
        return Ok(());
    }
    Err(common::argument_err())
}

fn run_workflow(dna_string: &str) -> Result<BTreeSet<String>> {
    let mut output = find_all_proteins(dna_string.as_bytes())?;
    output.append(&mut find_all_proteins(&common::dna::reverse_complement(
        dna_string.as_bytes(),
    )?)?);
    Ok(output)
}

fn find_all_proteins(dna_string: &[u8]) -> Result<BTreeSet<String>> {
    let mut output = BTreeSet::new();
    let starts = find_starts(dna_string)?;
    for start in starts {
        let mut protein = get_protein(dna_string, start)?;
        if protein.pop() != Some(END_CODON) {
            continue;
        }
        let protein = String::from_utf8(protein.to_vec())?;
        output.insert(protein);
    }
    Ok(output)
}

fn get_protein(dna_string: &[u8], start: usize) -> Result<Vec<u8>> {
    let mut protein = Vec::new();
    for substring in dna_string[start..].chunks_exact(3) {
        let protein_base = translate(substring)?;
        protein.push(protein_base);
        if protein_base == END_CODON {
            break;
        }
    }
    Ok(protein)
}

fn find_starts(dna_string: &[u8]) -> Result<Vec<usize>> {
    let mut starts = Vec::new();
    for (i, substring) in dna_string.windows(3).enumerate() {
        if translate(substring)? == START_CODON {
            starts.push(i);
        }
    }
    Ok(starts)
}

fn translate(substring: &[u8]) -> Result<u8> {
    match substring {
        s if s == b"GCT" || s == b"GCC" || s == b"GCA" || s == b"GCG" => Ok(b'A'),
        s if s == b"TGT" || s == b"TGC" => Ok(b'C'),
        s if s == b"GAT" || s == b"GAC" => Ok(b'D'),
        s if s == b"GAA" || s == b"GAG" => Ok(b'E'),
        s if s == b"TTT" || s == b"TTC" => Ok(b'F'),
        s if s == b"GGT" || s == b"GGC" || s == b"GGA" || s == b"GGG" => Ok(b'G'),
        s if s == b"CAT" || s == b"CAC" => Ok(b'H'),
        s if s == b"ATT" || s == b"ATC" || s == b"ATA" => Ok(b'I'),
        s if s == b"AAA" || s == b"AAG" => Ok(b'K'),
        s if s == b"TTA"
            || s == b"TTG"
            || s == b"CTT"
            || s == b"CTC"
            || s == b"CTA"
            || s == b"CTG" =>
        {
            Ok(b'L')
        }
        s if s == b"ATG" => Ok(b'M'),
        s if s == b"AAT" || s == b"AAC" => Ok(b'N'),
        s if s == b"CCT" || s == b"CCC" || s == b"CCA" || s == b"CCG" => Ok(b'P'),
        s if s == b"CAA" || s == b"CAG" => Ok(b'Q'),
        s if s == b"CGT"
            || s == b"CGC"
            || s == b"CGA"
            || s == b"CGG"
            || s == b"AGA"
            || s == b"AGG" =>
        {
            Ok(b'R')
        }
        s if s == b"TCT"
            || s == b"TCC"
            || s == b"TCA"
            || s == b"TCG"
            || s == b"AGT"
            || s == b"AGC" =>
        {
            Ok(b'S')
        }
        s if s == b"ACT" || s == b"ACC" || s == b"ACA" || s == b"ACG" => Ok(b'T'),
        s if s == b"GTT" || s == b"GTC" || s == b"GTA" || s == b"GTG" => Ok(b'V'),
        s if s == b"TGG" => Ok(b'W'),
        s if s == b"TAT" || s == b"TAC" => Ok(b'Y'),
        s if s == b"TAA" || s == b"TAG" || s == b"TGA" => Ok(b'-'),
        _ => Err(Error::new(
            ErrorKind::IO,
            &format!(
                "unrecognized substring: {}",
                String::from_utf8(substring.to_vec())?
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_workflow() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a str,
            expected: Result<BTreeSet<String>>,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            dna_string: "AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG",
            expected: Ok(BTreeSet::from(
                [
                    "M".to_owned(), 
                    "MGMTPRLGLESLLE".to_owned(), 
                    "MLLGSFRLIPKETLIQVAGSSPCNLS".to_owned(), 
                    "MTPRLGLESLLE".to_owned(),
                    ]
                )
            ),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                run_workflow(test_case.dna_string),
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_get_protein() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a [u8],
            start: usize,
            expected: Result<Vec<u8>>,
        }
        let test_cases = [TestCase {
            name: "Should translate dna to protein",
            dna_string: b"AATGTATTAA",
            start: 1,
            expected: Ok(b"MY-".to_vec()),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                get_protein(test_case.dna_string, test_case.start),
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_find_starts() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a [u8],
            expected: Result<Vec<usize>>,
        }
        let test_cases = [TestCase {
            name: "Should find starts",
            dna_string: b"ATGGCTATG",
            expected: Ok(vec![0, 6]),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                find_starts(test_case.dna_string),
                "{}",
                test_case.name
            );
        }
    }
}
