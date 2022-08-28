//! Enumerating k-mers Lexicographically (https://rosalind.info/problems/lexf/)
//!
//! # Problem
//!     Given: A collection of at most 10 symbols defining an ordered alphabet, and a positive
//!             integer n (n <= 10).
//!
//!     Return: All strings of length n that can be formed from the alphabet, ordered
//!             lexicographically (use the standard order of symbols in the English alphabet).
//!
//! # Sample Dataset
//!     A C G T
//!     2
//!
//! # Sample Output
//!     AA
//!     AC
//!     AG
//!     AT
//!     CA
//!     CC
//!     CG
//!     CT
//!     GA
//!     GC
//!     GG
//!     GT
//!     TA
//!     TC
//!     TG
//!     TT
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};
use itertools::Itertools;

pub const SUBCOMMAND: &str = "lexf";
const LINE_DELIMITER: &str = "\n";
const BASE_DELIMITER: &str = " ";

/// Return the subcommand for LEXF
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the lexf workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let (alphabet, n) = parse_input(input)?;
        let output = permute(&alphabet, n)?;
        for o in output {
            println!("{}", o);
        }
        return Ok(());
    }
    Err(common::argument_err())
}

/// Parse input
fn parse_input(input: String) -> Result<(Vec<u8>, usize)> {
    let input: Vec<&str> = input.split(LINE_DELIMITER).collect();
    if input.len() != 2 {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    let alphabet = input[0].replace(BASE_DELIMITER, "").as_bytes().to_vec();
    let n = input[1].parse::<usize>()?;
    Ok((alphabet, n))
}

/// Create cartesian product
fn permute(alphabet: &[u8], length: usize) -> Result<Vec<String>> {
    let alpha = alphabet.iter().map(|b| vec![b]);
    let mut output: Vec<Vec<&u8>> = alpha.clone().collect();
    for _ in 0..length - 1 {
        output = output
            .iter()
            .cartesian_product(alpha.clone())
            .map(|(l, r)| {
                let mut l = l.clone();
                let mut r = r.clone();
                l.append(&mut r);
                l
            })
            .collect();
    }
    let output = output
        .into_iter()
        .map(|x| String::from_utf8(x.into_iter().cloned().collect()))
        .collect::<std::result::Result<Vec<String>, _>>()?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        struct TestCase<'a> {
            name: &'a str,
            alphabet: &'a [u8],
            length: usize,
            expected: Result<Vec<String>>,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            alphabet: b"ACGT",
            length: 2,
            expected: Ok(vec![
                "AA".to_owned(),
                "AC".to_owned(),
                "AG".to_owned(),
                "AT".to_owned(),
                "CA".to_owned(),
                "CC".to_owned(),
                "CG".to_owned(),
                "CT".to_owned(),
                "GA".to_owned(),
                "GC".to_owned(),
                "GG".to_owned(),
                "GT".to_owned(),
                "TA".to_owned(),
                "TC".to_owned(),
                "TG".to_owned(),
                "TT".to_owned(),
            ]),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                permute(test_case.alphabet, test_case.length),
                "{}",
                test_case.name,
            );
        }
    }
}
