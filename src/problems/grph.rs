//! Overlap Graphs (https://rosalind.info/problems/grph/)
//!
//! # Problem
//!     Given: A collection of DNA strings in FASTA format having total length at most 10 kbp
//!
//!     Return: The adjacency list corresponding to O_3.  You may return edges in any order.
//!
//! # Sample Dataset
//!     >Rosalind_0498
//!     AAATAAA
//!     >Rosalind_2391
//!     AAATTTT
//!     >Rosalind_2323
//!     TTTTCCC
//!     >Rosalind_0442
//!     AAATCCC
//!     >Rosalind_5013
//!     GGGTGGG
//!
//! # Sample Output
//!     Rosalind_0498 Rosalind_2391
//!     Rosalind_0498 Rosalind_0442
//!     Rosalind_2391 Rosalind_2323
//!
use crate::common;
use crate::errors::Result;

pub const SUBCOMMAND: &str = "grph";

/// Return the subcommand for GRPH
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the grph workflow
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let file = std::fs::File::open(path)?;
        let reader = common::fasta::Reader::new(file);
        build_graph(reader)?;
        return Ok(());
    }
    Err(common::argument_err())
}

/// Build graph
fn build_graph<R>(reader: common::fasta::Reader<R>) -> Result<()>
where
    R: std::io::Read,
{
    let records: Vec<common::fasta::Record> = reader
        .iter()
        .collect::<Result<Vec<common::fasta::Record>>>()?;
    for seq1 in &records {
        for seq2 in &records {
            if seq1 == seq2 {
                continue;
            }
            if is_overlapped(&seq1.sequence, &seq2.sequence) {
                println!("{} {}", seq1.name, seq2.name);
            }
        }
    }
    Ok(())
}

/// Checks to see if the two sequences are overlapped
///
/// The two sequences are considered overlapped if the last 3 bases of seq1 is equal to the first 3
/// bases of seq2.
///
fn is_overlapped(seq1: &str, seq2: &str) -> bool {
    if seq1.len() < 3 || seq2.len() < 3 {
        return false;
    }
    let seq1_suffix = &seq1[seq1.len() - 3..];
    let seq2_prefix = &seq2[..3];

    seq1_suffix == seq2_prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_overlapped() {
        struct TestCase<'a> {
            name: &'a str,
            seq1: &'a str,
            seq2: &'a str,
            expected: bool,
        }
        let test_cases = [
            TestCase {
                name: "Should return true",
                seq1: "AAATAAA",
                seq2: "AAATTTT",
                expected: true,
            },
            TestCase {
                name: "Should return false",
                seq1: "AAAAAA",
                seq2: "TTTTTT",
                expected: false,
            },
        ];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                is_overlapped(test_case.seq1, test_case.seq2),
                "{}",
                test_case.name
            );
        }
    }
}
