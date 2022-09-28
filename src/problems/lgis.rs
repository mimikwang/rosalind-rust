//! Longest Increasing Subsequence
//!
//! # Problem
//!     Given: A positive integer n <= 10000 followed by a permutation pi of length n.
//!
//!     Return: A longest increasing subsequence of pi, followed by a longest decreasing
//!             subsequence of pi.
//!
//! # Sample Dataset
//!     5
//!     5 1 4 2 3
//!
//! # Sample Output
//!     1 2 3
//!     5 4 2
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "lgis";
const LINE_DELIMITER: &str = "\n";
const DELIMITER: &str = " ";

/// Return subcommand for LGIS
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the lgis workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?;
        let pi = parse_input(input)?;
        let longest_inc = longest(&pi, |left, right| left < right);
        let longest_dec = longest(&pi, |left, right| left > right);
        println!("{}", format_output(&longest_inc));
        println!("{}", format_output(&longest_dec));
        return Ok(());
    }
    Err(common::argument_err())
}

fn format_output(output: &[i64]) -> String {
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Parse input
fn parse_input(input: String) -> Result<Vec<i64>> {
    let input: Vec<&str> = input.split(LINE_DELIMITER).collect();
    if input.len() != 2 {
        return Err(Error::new(ErrorKind::IO, "invalid input"));
    }
    let pi = input[1]
        .split(DELIMITER)
        .map(|x| x.parse::<i64>())
        .collect::<std::result::Result<_, std::num::ParseIntError>>()?;
    Ok(pi)
}

/// Return the longest increasing or decreasing depending on comp_func
fn longest(pi: &[i64], comp_func: fn(i64, i64) -> bool) -> Vec<i64> {
    let order = get_order(pi, comp_func);
    order_to_result(pi, order)
}

/// Given pi, return an order vector
fn get_order(pi: &[i64], comp_func: fn(i64, i64) -> bool) -> Vec<usize> {
    let mut order = vec![0; pi.len()];
    for left_ind in (0..pi.len() - 1).rev() {
        for right_ind in (left_ind + 1..pi.len()).rev() {
            if comp_func(pi[left_ind], pi[right_ind]) && order[left_ind] <= order[right_ind] {
                order[left_ind] += 1;
            }
        }
    }
    order
}

/// Convert order to result
fn order_to_result(pi: &[i64], order: Vec<usize>) -> Vec<i64> {
    let mut result = vec![];
    let mut max = *order.iter().max().unwrap_or(&0) as i64;
    for (ind, o) in order.iter().enumerate() {
        if *o as i64 == max {
            result.push(pi[ind]);
            max -= 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        struct TestCase<'a> {
            name: &'a str,
            pi: &'a [i64],
            comp_func: fn(i64, i64) -> bool,
            expected: Vec<i64>,
        }
        let test_cases = [
            TestCase {
                name: "Should return the longest increasing",
                pi: &[5, 1, 4, 2, 3],
                comp_func: |left, right| left < right,
                expected: vec![1, 2, 3],
            },
            TestCase {
                name: "Should return the longest decreasing",
                pi: &[5, 1, 4, 2, 3],
                comp_func: |left, right| left > right,
                expected: vec![5, 4, 2],
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                longest(test_case.pi, test_case.comp_func),
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_get_order() {
        struct TestCase<'a> {
            name: &'a str,
            pi: &'a [i64],
            comp_func: fn(i64, i64) -> bool,
            expected: Vec<usize>,
        }
        let test_cases = [
            TestCase {
                name: "Should return order for ascending",
                pi: &[5, 1, 4, 2, 3],
                comp_func: |left, right| left < right,
                expected: vec![0, 2, 0, 1, 0],
            },
            TestCase {
                name: "Should return order for descending",
                pi: &[5, 1, 4, 2, 3],
                comp_func: |left, right| left > right,
                expected: vec![2, 0, 1, 0, 0],
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                get_order(test_case.pi, test_case.comp_func),
                "{}",
                test_case.name
            );
        }
    }

    #[test]
    fn test_order_to_result() {
        struct TestCase<'a> {
            name: &'a str,
            pi: &'a [i64],
            order: Vec<usize>,
            expected: Vec<i64>,
        }
        let test_cases = [
            TestCase {
                name: "Should return ascending",
                pi: &[5, 1, 4, 2, 3],
                order: vec![0, 2, 0, 1, 0],
                expected: vec![1, 2, 3],
            },
            TestCase {
                name: "Should return descending",
                pi: &[5, 1, 4, 2, 3],
                order: vec![2, 0, 1, 0, 0],
                expected: vec![5, 4, 2],
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                order_to_result(test_case.pi, test_case.order),
                "{}",
                test_case.name
            );
        }
    }
}
