use crate::errors::{Error, ErrorKind, Result};
use std::io::BufRead;

const NAME_PREFIX: &str = ">";

/// Reader for fasta files
pub struct Reader<R>
where
    R: std::io::Read,
{
    reader: std::io::BufReader<R>,
    line: String,
}

impl<R> Reader<R>
where
    R: std::io::Read,
{
    /// Constructor for fasta Reader
    pub fn new(reader: R) -> Self {
        Self {
            reader: std::io::BufReader::new(reader),
            line: String::new(),
        }
    }

    /// Read data into a record
    pub fn read(&mut self, record: &mut Record) -> Result<()> {
        if self.line.is_empty() {
            self.read_line()?;
        }
        if !is_name(&self.line) {
            return Err(Error::new(ErrorKind::IO, "invalid fasta format"));
        }
        record.name = get_name(&self.line);
        record.sequence = String::new();
        loop {
            self.read_line()?;
            if is_name(&self.line) {
                break;
            }
            record.sequence.push_str(&get_sequence(&self.line));
        }
        Ok(())
    }

    /// Returns an iterator
    pub fn iter(self) -> Records<R> {
        Records { reader: self }
    }

    /// Read line
    fn read_line(&mut self) -> Result<()> {
        self.line.clear();
        let bytes = self.reader.read_line(&mut self.line)?;
        if bytes == 0 {
            return Err(Error::new(ErrorKind::Eof, "end of file"));
        }
        Ok(())
    }
}

/// Check to see if line is a name line
fn is_name(line: &str) -> bool {
    line.starts_with(NAME_PREFIX)
}

/// Get name retrieves the name from a name line
fn get_name(line: &str) -> String {
    line.strip_prefix(NAME_PREFIX).unwrap_or("").trim().into()
}

/// Get sequence retrieves the sequence from the line
fn get_sequence(line: &str) -> String {
    line.trim().into()
}

/// Record is a fasta record
#[derive(Debug, Default, PartialEq)]
pub struct Record {
    pub name: String,
    pub sequence: String,
}

impl Record {
    /// Construct a new, empty record
    pub fn new() -> Self {
        Self::default()
    }
}

/// Type for iterating records
pub struct Records<R>
where
    R: std::io::Read,
{
    reader: Reader<R>,
}

impl<R> Iterator for Records<R>
where
    R: std::io::Read,
{
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut record = Record::new();
        match self.reader.read(&mut record) {
            Ok(()) => Some(Ok(record)),
            Err(err) if err.kind() == &ErrorKind::Eof => None,
            Err(err) => Some(Err(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_read() {
        let mut record = Record::new();
        let data: &[u8] = b">name\nACGT\nAAAA\n>name2\nAAGGTT\n";
        let mut reader = Reader::new(data);

        assert!(
            reader.read(&mut record).is_ok(),
            "{}",
            "Should read first record",
        );
        assert_eq!(
            String::from("name"),
            record.name,
            "{}",
            "Should read the name correctly",
        );
        assert_eq!(
            String::from("ACGTAAAA"),
            record.sequence,
            "{}",
            "Should read the sequence correctly",
        );

        let err = reader.read(&mut record).err().unwrap();
        assert_eq!(
            &ErrorKind::Eof,
            err.kind(),
            "{}",
            "Should read second record and return EOF",
        );
        assert_eq!(
            String::from("name2"),
            record.name,
            "{}",
            "Should read the name correctly",
        );
        assert_eq!(
            String::from("AAGGTT"),
            record.sequence,
            "{}",
            "Should read the sequence correctly",
        );
    }

    #[test]
    fn test_is_name() {
        assert_eq!(
            false,
            is_name("abcdefg"),
            "{}",
            "Should return false if line is not a name line"
        );
        assert_eq!(
            true,
            is_name(">abcdef"),
            "{}",
            "Should return true if line is a name line"
        );
    }

    #[test]
    fn test_get_name() {
        assert_eq!(
            String::from("abc"),
            get_name(">abc"),
            "{}",
            "Should parse name from name line",
        );
        assert_eq!(
            String::new(),
            get_name("abc"),
            "{}",
            "Should return an empty string if not a name line",
        );
    }

    #[test]
    fn test_get_sequence() {
        assert_eq!(
            String::from("ACGT"),
            get_sequence("ACGT\n"),
            "{}",
            "Should prase sequence from sequence line",
        );
    }
}
