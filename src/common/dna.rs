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

/// Translate DNA to protein
pub fn dna_to_protein(substring: &[u8]) -> Result<Option<u8>> {
    match substring {
        b"GCT" | b"GCC" | b"GCA" | b"GCG" => Ok(Some(b'A')),
        b"TGT" | b"TGC" => Ok(Some(b'C')),
        b"GAT" | b"GAC" => Ok(Some(b'D')),
        b"GAA" | b"GAG" => Ok(Some(b'E')),
        b"TTT" | b"TTC" => Ok(Some(b'F')),
        b"GGT" | b"GGC" | b"GGA" | b"GGG" => Ok(Some(b'G')),
        b"CAT" | b"CAC" => Ok(Some(b'H')),
        b"ATT" | b"ATC" | b"ATA" => Ok(Some(b'I')),
        b"AAA" | b"AAG" => Ok(Some(b'K')),
        b"TTA" | b"TTG" | b"CTT" | b"CTC" | b"CTA" | b"CTG" => Ok(Some(b'L')),
        b"ATG" => Ok(Some(b'M')),
        b"AAT" | b"AAC" => Ok(Some(b'N')),
        b"CCT" | b"CCC" | b"CCA" | b"CCG" => Ok(Some(b'P')),
        b"CAA" | b"CAG" => Ok(Some(b'Q')),
        b"CGT" | b"CGC" | b"CGA" | b"CGG" | b"AGA" | b"AGG" => Ok(Some(b'R')),
        b"TCT" | b"TCC" | b"TCA" | b"TCG" | b"AGT" | b"AGC" => Ok(Some(b'S')),
        b"ACT" | b"ACC" | b"ACA" | b"ACG" => Ok(Some(b'T')),
        b"GTT" | b"GTC" | b"GTA" | b"GTG" => Ok(Some(b'V')),
        b"TGG" => Ok(Some(b'W')),
        b"TAT" | b"TAC" => Ok(Some(b'Y')),
        b"TAA" | b"TAG" | b"TGA" => Ok(None),
        _ => Err(Error::new(
            ErrorKind::IO,
            &format!(
                "unrecognized substring: {}",
                String::from_utf8(substring.to_vec())?
            ),
        )),
    }
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
