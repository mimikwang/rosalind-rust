//! Enumerating Gene Orders (https://rosalind.info/problems/perm/)
//!
//! # Problem
//!
//!     Given: A positive integer n <= 7.
//!
//!     Return: The total number of permutations of length n, followed by a list of all such
//!             permutations (in any order).
//!
//! # Sample Dataset
//!     3
//!
//! # Sample Output
//!     6
//!     1 2 3
//!     1 3 2
//!     2 1 3
//!     3 1 2
//!     3 2 1
//!
use crate::common;
use crate::errors::Result;
use itertools::Itertools;

pub const SUBCOMMAND: &str = "perm";

/// Return the subcommand for PERM
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the perm workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let input = common::load_simple(path)?.parse::<usize>()?;
        let perms = permutations(input);
        println!("{}", perms.len());
        for perm in perms {
            println!("{}", format_line(&perm));
        }
        return Ok(());
    }
    Err(common::argument_err())
}

fn format_line(line: &[usize]) -> String {
    if line.is_empty() {
        return "".to_owned();
    }
    let mut output = format!("{}", line[0]);
    for entry in line.iter().skip(1) {
        output = format!("{} {}", output, entry);
    }
    output
}

fn permutations(n: usize) -> Vec<Vec<usize>> {
    (1..=n).permutations(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        struct TestCase<'a> {
            name: &'a str,
            n: usize,
            expected: Vec<Vec<usize>>,
        }
        let test_cases = [TestCase {
            name: "Sample Dataset",
            n: 3,
            expected: vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                permutations(test_case.n),
                "{}",
                test_case.name
            );
        }
    }
}
