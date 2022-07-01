//! Calculating Expected Offspring (https://rosalind.info/problems/iev/)
//!
//! # Problem
//!
//!     Given: Six nonnegative integers, each of which does not exceed 20,000.  The integers
//!             correspond to the number of couples in a population possessing each genotype
//!             pairing for a given factor.  In order, the six given integers represent the
//!             number of couples having the following genotypes:
//!
//!             1. AA-AA
//!             2. AA-Aa
//!             3. AA-aa
//!             4. Aa-Aa
//!             5. Aa-aa
//!             6. aa-aa
//!
//!     Return: The expected number of offspring displaying the dominant phenotype in the next
//!             generation, under the assumption that every couple has exactly two offspring.
//!
//! # Sample Dataset
//!     1 0 0 1 0 1
//!
//! # Sample Output
//!     3.5
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "iev";
const DELIMITER: &str = " ";

/// Return the subcommand for IEV
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the iev workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let input = parse_input(input)?;
        let output = calculate_expected(input);
        println!("{}", output);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Parse input into an array of usize
fn parse_input(input: String) -> Result<[usize; 6]> {
    let split: Vec<&str> = input.split(DELIMITER).collect();
    if split.len() != 6 {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    let mut output: [usize; 6] = [0; 6];
    for (i, s) in split.iter().enumerate() {
        output[i] = s.parse::<usize>()?;
    }
    Ok(output)
}

fn calculate_expected(input: [usize; 6]) -> f64 {
    let prob: [f64; 6] = [2.0, 2.0, 2.0, 1.5, 1.0, 0.0];
    input
        .iter()
        .zip(prob.iter())
        .map(|(&i, &p)| i as f64 * p)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_expected() {
        struct TestCase<'a> {
            name: &'a str,
            input: [usize; 6],
            expected: f64,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            input: [1, 0, 0, 1, 0, 1],
            expected: 3.5,
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                calculate_expected(test_case.input),
                "{}",
                test_case.name
            );
        }
    }
}
