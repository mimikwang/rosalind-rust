//! Calculating Protein Mass (https://rosalind.info/problems/prtm/)
//!
//! # Problem
//!
//!     Given: A protein string P at length at most 1000 aa.
//!
//!     Return: The total weight of P.  Consult the monoisotopic mass table.
//!
//! # Sample Dataset
//!     SKADYEK
//!
//! # Sample Output
//!     821.392
//!
use crate::common;
use crate::errors::Result;

pub const SUBCOMMAND: &str = "prtm";

/// Return the command for PRTM
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the prtm workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let protein = common::load_simple(path)?;
        println!("{}", mass(protein.as_bytes()));
        return Ok(());
    }
    Err(common::argument_err())
}

/// Calculate the mass of a protein string
fn mass(protein: &[u8]) -> f64 {
    protein.iter().map(lookup).sum()
}

/// Return the monoisotopic mass table
fn lookup(protein_base: &u8) -> f64 {
    match protein_base {
        b'A' => 71.03711,
        b'C' => 103.00919,
        b'D' => 115.02694,
        b'E' => 129.04259,
        b'F' => 147.06841,
        b'G' => 57.02146,
        b'H' => 137.05891,
        b'I' => 113.08406,
        b'K' => 128.09496,
        b'L' => 113.08406,
        b'M' => 131.04049,
        b'N' => 114.04293,
        b'P' => 97.05276,
        b'Q' => 128.05858,
        b'R' => 156.10111,
        b'S' => 87.03203,
        b'T' => 101.04768,
        b'V' => 99.06841,
        b'W' => 186.07931,
        b'Y' => 163.06333,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass() {
        struct TestCase<'a> {
            name: &'a str,
            protein: &'a [u8],
            expected: f64,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            protein: b"SKADYEK",
            expected: 821.392,
        }];
        for test_case in test_cases {
            assert!(
                (test_case.expected - mass(test_case.protein)).abs() < 0.01,
                "{}",
                test_case.name,
            );
        }
    }
}
