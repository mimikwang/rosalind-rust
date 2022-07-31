use crate::errors::{Error, ErrorKind, Result};

/// Complement a DNA base
pub fn complement(base: &u8) -> Result<u8> {
    match base {
        b'A' => Ok(b'T'),
        b'T' => Ok(b'A'),
        b'C' => Ok(b'G'),
        b'G' => Ok(b'C'),
        _ => Err(Error::new(ErrorKind::IO, "not a dna base")),
    }
}

/// Reverse complement a dna string
pub fn reverse_complement(dna_string: &[u8]) -> Result<Vec<u8>> {
    dna_string.iter().rev().map(complement).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_complement() {
        struct TestCase<'a> {
            name: &'a str,
            dna_string: &'a [u8],
            expected: Result<Vec<u8>>,
        }
        let test_cases = [TestCase {
            name: "Should reverse complement",
            dna_string: b"AGTC",
            expected: Ok(b"GACT".to_vec()),
        }];
        for test_case in test_cases {
            assert_eq!(
                test_case.expected,
                reverse_complement(test_case.dna_string),
                "{}",
                test_case.name
            );
        }
    }
}
