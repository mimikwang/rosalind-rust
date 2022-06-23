//! Consensus and Profile (https://rosalind.info/problems/cons/)
//!
//! # Problem
//!
//!     Given: A collection of at most 10 DNA strings of equal length (at most 1 kbp) in FASTA
//!             format
//!
//!     Return: A consensus string and profile matrix for the collection.  (If several possible
//!             consensus strings exist, you may return any one of them.)
//!
//! # Sample Dataset
//!     >Rosalind_1
//!     ATCCAGCT
//!     >Rosalind_2
//!     GGGCAACT
//!     >Rosalind_3
//!     ATGGATCT
//!     >Rosalind_4
//!     AAGCAACC
//!     >Rosalind_5
//!     TTGGAACT
//!     >Rosalind_6
//!     ATGCCATT
//!     >Rosalind_7
//!     ATGGCACT
//!
//! # Sample Output
//!     ATGCAACT
//!     A: 5 1 0 0 5 5 0 0
//!     C: 0 0 1 4 2 0 6 1
//!     G: 1 1 6 3 0 1 0 0
//!     T: 1 5 0 0 0 1 1 6
//!
use crate::common;
use crate::errors::Result;
use std::collections::BTreeMap;

pub const SUBCOMMAND: &str = "cons";

/// Return subcommand for CONS
pub fn command() -> clap::Command<'static> {
    common::subcommand_file(SUBCOMMAND)
}

/// Run the cons workflow
///
/// Run the workflow by loading the fasta file and incrementing a counter per sequence.
///
pub fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(path) = matches.value_of(common::FILE_ARG) {
        let reader = common::fasta::Reader::new(std::fs::File::open(path)?);
        let mut counter = Counter::new(0);
        for record in reader.iter() {
            let sequence = record?.sequence;
            if counter.sequence_length == 0 {
                counter = Counter::new(sequence.len());
            }
            counter.update(&sequence);
        }
        println!("{}", counter.format_output()?);
        return Ok(());
    }
    Err(common::argument_err())
}

/// Counter for bases
#[derive(Debug, PartialEq)]
struct Counter {
    counter: BTreeMap<u8, Vec<usize>>,
    sequence_length: usize,
}

impl Counter {
    /// Construct a new counter
    fn new(sequence_length: usize) -> Self {
        Self {
            counter: BTreeMap::from([
                (b'A', vec![0; sequence_length]),
                (b'C', vec![0; sequence_length]),
                (b'G', vec![0; sequence_length]),
                (b'T', vec![0; sequence_length]),
            ]),
            sequence_length,
        }
    }

    /// Update counter with a sequence
    fn update(&mut self, sequence: &str) {
        let seq = sequence.as_bytes();
        for index in 0..std::cmp::max(self.sequence_length, seq.len()) {
            if let Some(counter) = self.counter.get_mut(&seq[index]) {
                counter[index] += 1;
            }
        }
    }

    /// Return the consensus sequence
    fn consensus(&self) -> Result<String> {
        let mut consensus = vec![b'A'; self.sequence_length];
        for (index, base) in consensus.iter_mut().enumerate().take(self.sequence_length) {
            let mut max_count = 0;
            for b in [b'A', b'C', b'G', b'T'] {
                let count = self
                    .counter
                    .get(&b)
                    .unwrap_or(&vec![0; self.sequence_length])[index];
                if count > max_count {
                    *base = b;
                    max_count = count;
                }
            }
        }
        let consensus = std::str::from_utf8(&consensus)?;
        Ok(consensus.to_owned())
    }

    /// Format the counter output
    fn format_output(&self) -> Result<String> {
        let mut output = self.consensus()?;
        for base in [b'A', b'C', b'G', b'T'] {
            output = format!("{}\n{}:", output, base as char);
            for count in self
                .counter
                .get(&base)
                .unwrap_or(&vec![0; self.sequence_length])
            {
                output = format! {"{} {}", output, count};
            }
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_update() {
        let mut counter = Counter::new(3);
        counter.update("AGC");
        assert_eq!(
            Counter {
                counter: BTreeMap::from([
                    (b'A', vec![1, 0, 0]),
                    (b'C', vec![0, 0, 1]),
                    (b'G', vec![0, 1, 0]),
                    (b'T', vec![0, 0, 0]),
                ]),
                sequence_length: 3,
            },
            counter,
        );
    }

    #[test]
    fn test_counter_consensus() {
        let counter = Counter {
            counter: BTreeMap::from([
                (b'A', vec![5, 1, 0, 0, 5, 5, 0, 0]),
                (b'C', vec![0, 0, 1, 4, 2, 0, 6, 1]),
                (b'G', vec![1, 1, 6, 3, 0, 1, 0, 0]),
                (b'T', vec![1, 5, 0, 0, 0, 1, 1, 6]),
            ]),
            sequence_length: 8,
        };
        assert_eq!(counter.consensus(), Ok("ATGCAACT".to_owned()));
    }
}
