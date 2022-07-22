//! Independent Alleles (https://rosalind.info/problems/lia/)
//!
//! # Problem
//!     Given: Two positive integers k (k <= 7) and N (N <= 2^k).  In this problem, we begin with
//!             Tom, who in the 0th generation has genotype Aa Bb.  Tom has two children in the 1st
//!             generation, each of whom has two children, and so on.  Each organism always mates
//!             with an organism having genotype Aa Bb.
//!
//!     Return: The probability that at least N Aa Bb organisms will belong to the k-th generation
//!             of Tom's family tree (don't count the Aa Bb mates at each level).  Assume that
//!             Mendel's second law holds for the factors.
//!
//! # Sample Dataset
//!     2 1
//!
//! # Sample Output
//!     0.684
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;

pub const SUBCOMMAND: &str = "lia";
const DELIMITER: &str = " ";

/// Return the subcommand for LIA
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the lia workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let (k, n) = parse_input(input)?;
        let output = prob(k, n)?;
        println!("{}", output);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Parse input
fn parse_input(input: String) -> Result<(u32, u32)> {
    let input: Vec<&str> = input.split(DELIMITER).collect();
    if input.len() != 2 {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    let k = input[0].parse::<u32>()?;
    let n = input[1].parse::<u32>()?;
    Ok((k, n))
}

/// Calculate the probability
fn prob(k: u32, n: u32) -> Result<f64> {
    let mut prob = 0.0;
    let total = 2u32.pow(k);
    for i in n..=total {
        let left = binom_coeff(total, i)?;
        let right = (0.25f64).powf(i as f64) * (0.75f64).powf((total - i) as f64);
        prob += left as f64 * right;
    }
    Ok(prob)
}

/// Caclulate the binomial coefficient
fn binom_coeff(n: u32, k: u32) -> Result<u128> {
    let coeff = factorial(n) / (factorial(k) * factorial(n - k));
    coeff
        .to_u128()
        .ok_or_else(|| Error::new(ErrorKind::User, "input too large"))
}

/// Calculate the factorial
fn factorial(n: u32) -> BigUint {
    let mut output = BigUint::from(1u32);
    for i in 1..=n {
        output *= BigUint::from(i);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prob() {
        struct TestCase<'a> {
            name: &'a str,
            k: u32,
            n: u32,
            expected: f64,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            k: 2,
            n: 1,
            expected: 0.684,
        }];
        for test_case in test_cases {
            let actual = prob(test_case.k, test_case.n);
            let actual = actual.unwrap();
            assert!(
                (test_case.expected - actual).abs() < 0.01,
                "{}",
                test_case.name,
            );
        }
    }

    #[test]
    fn test_factorial() {
        struct TestCase<'a> {
            name: &'a str,
            n: u32,
            expected: BigUint,
        }
        let test_cases = [
            TestCase {
                name: "Should return 1",
                n: 0,
                expected: BigUint::from(1 as u32),
            },
            TestCase {
                name: "Should return 1",
                n: 1,
                expected: BigUint::from(1 as u32),
            },
            TestCase {
                name: "Should return 24",
                n: 4,
                expected: BigUint::from(24 as u32),
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                factorial(test_case.n),
                "{}",
                test_case.name,
            );
        }
    }
}
