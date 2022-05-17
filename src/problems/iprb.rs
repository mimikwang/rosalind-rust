//! Mendel's First Law (https://rosalind.info/problems/iprb/)
//!
//! # Problem
//!
//!     Given: Three positive integers k, m, and n, representing a population containing k + m + n
//!             organisms: k individuals are homozygous dominant for a factor, m are heterozygous,
//!             and n are homozygous recessive.
//!
//!     Return: The probability that two randomly selected mating organisms will produce an
//!             individual possessing a dominant allele (and thus displaying the dominant
//!             phenotype).  Assume that any two organisms can mate.
//!
//! # Sample Dataset
//!     2 2 2
//!
//! # Sample Output
//!     0.78333
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};
use itertools::Itertools;

pub const SUBCOMMAND: &str = "iprb";
const DELIMITER: &str = " ";

/// Return the command for IPRB
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the iprob workflow
///
/// Run the workflow by loading the data from the text file, parsing the k, m, and n integers,
/// running the probability algorithm, and then printing the results.
///
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let (k, m, n) = parse_input(input)?;
        let prob = calc_probability(k, m, n);
        println!("{}", prob);
        return Ok(());
    }
    Err(Error::new(
        ErrorKind::User,
        &format!("{} argument required", common::FILE_ARG),
    ))
}

/// Parse input
///
/// Parse input into k, m, and n and return an error if the input is invalid
///
fn parse_input(input: String) -> Result<(usize, usize, usize)> {
    let input: Vec<&str> = input.split(DELIMITER).collect();
    if input.len() != 3 {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    let k = input[0].parse::<usize>()?;
    let m = input[1].parse::<usize>()?;
    let n = input[2].parse::<usize>()?;
    Ok((k, m, n))
}

/// Allele pairs
#[derive(Debug, Clone, Copy, PartialEq)]
enum AllelePairs {
    HomoDom,
    HomoRec,
    Hetero,
}

/// Calculate probability of two randomly selected mating organisms will produce an individual
/// possessing a dominant allele.
fn calc_probability(k: usize, m: usize, n: usize) -> f64 {
    let combinations = pairs(k, m, n);
    let probs = combinations.iter().map(|&x| probability(x));
    let mut sum = 0.0;
    let mut count = 0.0;
    probs.for_each(|prob| {
        sum += prob;
        count += 1.0;
    });
    sum / count
}

/// Return all possible pair combinations
fn pairs(k: usize, m: usize, n: usize) -> Vec<(AllelePairs, AllelePairs)> {
    [
        vec![AllelePairs::HomoDom; k],
        vec![AllelePairs::HomoRec; n],
        vec![AllelePairs::Hetero; m],
    ]
    .concat()
    .into_iter()
    .combinations(2)
    .map(|x| (x[0], x[1]))
    .collect()
}

/// Return the probability of the offspring having at least one dominant allele
///
/// For example, if the mother and father are both homozogous dominant, then the probability of
/// the offspring having at least one dominant allele is 100%.  If the mother and father are both
/// homozygous recessive, then the probability of the offspring having at least one dominant allele
/// is 0%.
///
fn probability(alleles: (AllelePairs, AllelePairs)) -> f64 {
    match alleles {
        (AllelePairs::HomoDom, _) => 1.0,
        (_, AllelePairs::HomoDom) => 1.0,
        (AllelePairs::Hetero, AllelePairs::Hetero) => 0.75,
        (AllelePairs::Hetero, AllelePairs::HomoRec) => 0.5,
        (AllelePairs::HomoRec, AllelePairs::Hetero) => 0.5,
        (AllelePairs::HomoRec, AllelePairs::HomoRec) => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_probability() {
        struct TestCase<'a> {
            name: &'a str,
            k: usize,
            m: usize,
            n: usize,
            expected: f64,
        }
        let test_cases = [
            TestCase {
                name: "Should work on sample dataset",
                k: 2,
                m: 2,
                n: 2,
                expected: 0.78333,
            },
            TestCase {
                name: "Should work on sample dataset",
                k: 26,
                m: 24,
                n: 19,
                expected: 0.8005115,
            },
        ];
        for test_case in test_cases {
            assert!(
                (test_case.expected - calc_probability(test_case.k, test_case.m, test_case.n))
                    .abs()
                    < 0.0001,
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_pairs() {
        struct TestCase<'a> {
            name: &'a str,
            k: usize,
            m: usize,
            n: usize,
            expected: Vec<(AllelePairs, AllelePairs)>,
        }
        let test_cases = [TestCase {
            name: "Should return the the correct pairs",
            k: 1,
            m: 1,
            n: 1,
            expected: vec![
                (AllelePairs::HomoDom, AllelePairs::HomoRec),
                (AllelePairs::HomoDom, AllelePairs::Hetero),
                (AllelePairs::HomoRec, AllelePairs::Hetero),
            ],
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                pairs(test_case.k, test_case.m, test_case.n),
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_probability() {
        struct TestCase<'a> {
            name: &'a str,
            alleles: (AllelePairs, AllelePairs),
            expected: f64,
        }
        let test_cases = [
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::HomoDom, AllelePairs::HomoDom),
                expected: 1.0,
            },
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::HomoDom, AllelePairs::HomoRec),
                expected: 1.0,
            },
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::HomoDom, AllelePairs::Hetero),
                expected: 1.0,
            },
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::Hetero, AllelePairs::HomoDom),
                expected: 1.0,
            },
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::Hetero, AllelePairs::HomoRec),
                expected: 0.5,
            },
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::Hetero, AllelePairs::Hetero),
                expected: 0.75,
            },
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::HomoRec, AllelePairs::HomoDom),
                expected: 1.0,
            },
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::HomoRec, AllelePairs::HomoRec),
                expected: 0.0,
            },
            TestCase {
                name: "Should return the right probability",
                alleles: (AllelePairs::HomoRec, AllelePairs::Hetero),
                expected: 0.5,
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                probability(test_case.alleles),
                "{}",
                test_case.name
            );
        }
    }
}
