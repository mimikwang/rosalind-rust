//! Inferring mRNA from Protein (https://rosalind.info/problems/mrna/)
//!
//! # Problem
//!     Given: A protein string of length at most 100 aa.
//!
//!     Return: The total number of different RNA strings from which the protein chould have been
//!             translated, modulo 1,000,000.  (Don't neglect the importance of the stop codon in
//!             protein translation.)
//!
//! # Sample Dataset
//!     MA
//!
//! # Sample Output
//!     12
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "mrna";

/// Return subcommand for MRNA
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the mrna workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let protein_string = common::load_simple(path)?;
        let output = total(&protein_string)?;
        println!("{}", output);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Calculate the total number of different RNA strings modulo 1,000,000
fn total(protein_string: &str) -> Result<u64> {
    if protein_string.is_empty() {
        return Ok(0);
    }
    let mut output = 1;
    for protein in protein_string.as_bytes() {
        let possibilities = rna_possbilities(protein)?;
        output *= possibilities;
        if output > 1_000_000 {
            output %= 1_000_000;
        }
    }
    Ok(output * 3)
}

/// Given a protein, return the number of possible RNA strings based on the RNA codon table
fn rna_possbilities(protein: &u8) -> Result<u64> {
    match protein {
        b'A' => Ok(4),
        b'C' => Ok(2),
        b'D' => Ok(2),
        b'E' => Ok(2),
        b'F' => Ok(2),
        b'G' => Ok(4),
        b'H' => Ok(2),
        b'I' => Ok(3),
        b'K' => Ok(2),
        b'L' => Ok(6),
        b'M' => Ok(1),
        b'N' => Ok(2),
        b'P' => Ok(4),
        b'Q' => Ok(2),
        b'R' => Ok(6),
        b'S' => Ok(6),
        b'T' => Ok(4),
        b'V' => Ok(4),
        b'W' => Ok(1),
        b'Y' => Ok(2),
        _ => Err(Error::new(ErrorKind::IO, "unrecognized protein base")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        struct TestCase<'a> {
            name: &'a str,
            protein_string: &'a str,
            expected: Result<u64>,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            protein_string: "MA",
            expected: Ok(12),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                total(test_case.protein_string),
                "{}",
                test_case.name,
            );
        }
    }
}
