//! Rabbits and Recurrence Relations (https://rosalind.info/problems/fib/)
//!
//! # Problem
//!     Given: Positive integers n <= 40 and k <= 5
//!
//!     Return: The total number of rabbit pairs that will be present after n months, if we begin
//!             with 1 pair and in each generation, every pair of reproduction-age rabbits produces
//!             a litter of k rabbit pairs (instead of only 1 pair).
//!
//! # Sample Dataset
//!     5 3
//!
//! # Sample Output
//!     19
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "fib";
const DELIMITER: &str = " ";

/// Return the command for FIB
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the fib workflow
///
/// Run the workflow by loading the data from the text file, parsing the n and k integers, running
/// the population counting algorithm, and then printing the results.
///
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let (n, k) = parse_input(input)?;
        let mut population = Population::new(k);
        population.advance_n(n);
        println!("{}", population.total());
        return Ok(());
    }
    Err(Error::new(
        ErrorKind::User,
        &format!("{} argument required", common::FILE_ARG),
    ))
}

/// Parse input
///
/// Parse input into k and n and return an error if the input is invalid
///
fn parse_input(input: String) -> Result<(i64, i64)> {
    let input: Vec<&str> = input.split(DELIMITER).collect();
    if input.len() != 2 {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    let n = input[0].parse::<i64>()?;
    let k = input[1].parse::<i64>()?;
    Ok((n, k))
}

/// Population represents the rabbit populations
struct Population {
    /// Number of newborn rabbit pairs for the current cycle
    newborn: i64,
    /// Number of reproductive rabbit pairs for the current cycle
    reproductive: i64,
    /// The number of rabbit pairs producted by each reproductive pairs per cycle
    k: i64,
}

impl Population {
    /// Constructor for Population
    fn new(k: i64) -> Self {
        Self {
            newborn: 1,
            reproductive: 0,
            k,
        }
    }

    /// Total number of rabbit pairs in the population
    fn total(&self) -> i64 {
        self.newborn + self.reproductive
    }

    /// Advances one cycle
    fn advance(&mut self) {
        let newborn = self.reproductive * self.k;
        self.reproductive = self.total();
        self.newborn = newborn;
    }

    /// Advance n cycles
    fn advance_n(&mut self, n: i64) {
        for _ in 0..n - 1 {
            self.advance()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        struct TestCase<'a> {
            name: &'a str,
            input: String,
            expected: (i64, i64),
            expect_error: bool,
        }
        let test_cases = [
            TestCase {
                name: "Should parse input",
                input: "5 32".into(),
                expected: (5, 32),
                expect_error: false,
            },
            TestCase {
                name: "Should return an error if there are too many input variables",
                input: "12 32 41".into(),
                expected: (0, 0),
                expect_error: true,
            },
            TestCase {
                name: "Should return an error if the variables are not integers",
                input: "a 32".into(),
                expected: (0, 0),
                expect_error: true,
            },
        ];
        for test_case in test_cases {
            let actual = parse_input(test_case.input);
            if test_case.expect_error {
                assert!(actual.is_err(), "{}", test_case.name);
            } else {
                assert_eq!(Ok(test_case.expected), actual, "{}", test_case.name);
            }
        }
    }

    #[test]
    fn test_population_advance_n() {
        let mut population = Population::new(3);
        population.advance_n(5);
        assert_eq!(19, population.total(), "{}", "Sample Dataset from problem");
    }
}
