//! Translating RNA into Protein (https://rosalind.info/problems/prot/)
//!
//! # Problem
//!
//!     Given: An RNA String s corresponding to a strand of mRNA (of length n at most 10 kbp).
//!
//!     Return: The protein string encoded by s.
//!
//! # Sample Dataset
//!     AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA
//!
//! # Sample Output
//!     MAMAPRTEINSTRING
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "prot";

/// Return the command for PROT
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the prot workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let rna_string = common::load_simple(path)?;
        let translated = translate(rna_string.as_bytes())?;
        println!("{}", translated);
        return Ok(());
    }
    Err(Error::new(
        ErrorKind::User,
        &format!("{} argument required", common::FILE_ARG),
    ))
}

/// Translate RNA string to protein
fn translate(rna_string: &[u8]) -> Result<String> {
    let mut protein = Vec::new();
    for codon in rna_string.chunks(3) {
        let p = translate_codon(codon)?;
        if let Some(x) = p {
            protein.push(x);
        } else {
            break;
        }
    }
    let output = String::from_utf8(protein)?;
    Ok(output)
}

/// Translate Codon to Protein
fn translate_codon(codon: &[u8]) -> Result<Option<u8>> {
    let protein: Option<u8> = match codon {
        b"GCU" | b"GCC" | b"GCA" | b"GCG" => Some(b'A'),
        b"UGU" | b"UGC" => Some(b'C'),
        b"GAU" | b"GAC" => Some(b'D'),
        b"GAA" | b"GAG" => Some(b'E'),
        b"UUU" | b"UUC" => Some(b'F'),
        b"GGU" | b"GGC" | b"GGA" | b"GGG" => Some(b'G'),
        b"CAU" | b"CAC" => Some(b'H'),
        b"AUU" | b"AUC" | b"AUA" => Some(b'I'),
        b"AAA" | b"AAG" => Some(b'K'),
        b"UUA" | b"UUG" | b"CUU" | b"CUC" | b"CUA" | b"CUG" => Some(b'L'),
        b"AUG" => Some(b'M'),
        b"AAU" | b"AAC" => Some(b'N'),
        b"CCU" | b"CCC" | b"CCA" | b"CCG" => Some(b'P'),
        b"CAA" | b"CAG" => Some(b'Q'),
        b"CGU" | b"CGC" | b"CGA" | b"CGG" | b"AGA" | b"AGG" => Some(b'R'),
        b"UCU" | b"UCC" | b"UCA" | b"UCG" | b"AGU" | b"AGC" => Some(b'S'),
        b"ACU" | b"ACC" | b"ACA" | b"ACG" => Some(b'T'),
        b"GUU" | b"GUC" | b"GUA" | b"GUG" => Some(b'V'),
        b"UGG" => Some(b'W'),
        b"UAU" | b"UAC" => Some(b'Y'),
        b"UAA" | b"UAG" | b"UGA" => None,
        _ => {
            return Err(Error::new(ErrorKind::User, "invalid codon"));
        }
    };
    Ok(protein)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        struct TestCase<'a> {
            name: &'a str,
            rna_string: &'a [u8],
            expected: Result<String>,
        }
        let test_cases = [
            TestCase {
                name: "Sample Dataset from problem",
                rna_string: b"AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA",
                expected: Ok("MAMAPRTEINSTRING".to_owned()),
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                translate(test_case.rna_string),
                "{}",
                test_case.name,
            );
        }
    }
}