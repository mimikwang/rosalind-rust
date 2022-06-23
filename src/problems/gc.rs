//! Computing GC Content (https://rosalind.info/problems/gc/)
//!
//! # Problem
//!
//!     Given: At most 10 DNA strings in FASTA format (of length at most 1 kbp each.)
//!
//!     Return: The ID of the string having the highest GC-content, followed by the GC-content of
//!             that string.  Rosalind allows for a default error of 0.001 in all decimal answers
//!             unless otherwise stated; pleae see the note on absolute error below.
//!
//! # Sample Dataset
//!     >Rosalind_6404
//!     CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
//!     TCCCACTAATAATTCTGAGG
//!     >Rosalind_5959
//!     CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
//!     ATATCCATTTGTCAGCAGACACGC
//!     >Rosalind_0808
//!     CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
//!     TGGGAACCTGCGGGCAGTAGGTGGAAT
//!
//! # Sample Output
//!     Rosalind_0808
//!     60.919540
//!
use crate::common;
use crate::errors::{Error, ErrorKind, Result};

pub const SUBCOMMAND: &str = "gc";

/// Return the subommand for GC
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the gc workflow
///
/// Run the workflow by loading the fasta records, calculating the gc, finding the entry with
/// the maximum gc, and printing it.
///
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let file = std::fs::File::open(path)?;
        let reader = common::fasta::Reader::new(file);
        let mut gc = calculate_gc(reader)?;
        let max = max_gc(&mut gc).ok_or_else(|| Error::new(ErrorKind::IO, "invalid input"))?;
        println!("{}\n{:.6}", max.0, max.1);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Get record with highest GC content
fn max_gc(gc: &mut Vec<(String, f64)>) -> Option<&(String, f64)> {
    gc.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    gc.first()
}

/// Calculate the GC of all records
fn calculate_gc<R>(reader: common::fasta::Reader<R>) -> Result<Vec<(String, f64)>>
where
    R: std::io::Read,
{
    let mut output = Vec::new();
    for result in reader.iter() {
        let record = result?;
        output.push((record.name, get_gc(record.sequence)))
    }
    Ok(output)
}

/// Get the GC content as a formatted string
fn get_gc(sequence: String) -> f64 {
    if sequence.is_empty() {
        return 0.0;
    }
    let gc_count: f64 = sequence
        .chars()
        .map(|base| if base == 'G' || base == 'C' { 1.0 } else { 0.0 })
        .sum();

    100.0 * (gc_count / sequence.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_gc() {
        let mut gc = vec![
            (String::from("abc"), 10.1),
            (String::from("aab"), 30.1),
            (String::from("aaa"), 30.1),
        ];
        assert_eq!(
            Some(&(String::from("aab"), 30.1)),
            max_gc(&mut gc),
            "{}",
            "Should return entry with the largest GC",
        );

        let mut gc = vec![];
        assert_eq!(
            None,
            max_gc(&mut gc),
            "{}",
            "Should return None on an empty vec",
        )
    }

    #[test]
    fn test_get_gc() {
        struct TestCase<'a> {
            name: &'a str,
            sequence: String,
            expected: f64,
        }
        let test_cases = [
            TestCase {
                name: "Should calculate GC correctly",
                sequence: "GGCCAAT".into(),
                expected: 57.142857,
            },
            TestCase {
                name: "Should return 0 for an empty sequence",
                sequence: "".into(),
                expected: 0.0,
            },
        ];
        for test_case in test_cases {
            assert!(
                (test_case.expected - get_gc(test_case.sequence)).abs() < 0.01,
                "{}",
                test_case.name
            );
        }
    }
}
