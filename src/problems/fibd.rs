//! Mortal Fibonacci Rabbits (https://rosalind.info/problems/fibd/)
//!
//! # Problem
//!     Given: Positive integers n <= 100 and m <= 20
//!
//!     Return: The total number of pairs of rabbits that will remain after the nth month if all
//!             rabbits live for m months.
//!
//! # Sample Dataset
//!     6 3
//!
//! # Sample Output
//!     4
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "fibd";
const DELIMITER: &str = " ";

/// Return the command for FIBD
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the fibd workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let (n, m) = parse_input(input)?;
        let output = simulate(n, m);
        println!("{}", output);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Parse input
fn parse_input(input: String) -> Result<(u128, u128)> {
    let input: Vec<&str> = input.split(DELIMITER).collect();
    if input.len() != 2 {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    let n = input[0].parse::<u128>()?;
    let m = input[1].parse::<u128>()?;
    Ok((n, m))
}

/// Given lifespan m and n number of months, return the total number of pairs of rabbits
fn simulate(n: u128, m: u128) -> u128 {
    // If n months is less than 2, then return 1
    if n <= 2 {
        return 1;
    }
    let mut tracker = vec![1, 1];
    for month in 0..n - 2 {
        let mut current = tracker.iter().rev().take(2).sum();
        match month {
            month if month == m - 2 => current -= 1,
            month if month > m - 2 => {
                current -= *tracker.get(tracker.len() - 1 - m as usize).unwrap_or(&0);
            }
            _ => (),
        }
        tracker.push(current);
    }
    *tracker.last().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        struct TestCase<'a> {
            name: &'a str,
            n: u128,
            m: u128,
            expected: u128,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            n: 6,
            m: 3,
            expected: 4,
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                simulate(test_case.n, test_case.m),
                "{}",
                test_case.name,
            );
        }
    }
}
