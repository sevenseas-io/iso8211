use crate::{DirectoryEntry, Leader, ReadResult, Reader, FIELD_TERMINATOR};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct Directory {
    entries: Vec<DirectoryEntry>,
}

impl Directory {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>, leader: &Leader) -> ReadResult<Directory> {
        let mut entries: Vec<DirectoryEntry> = Vec::new();

        while reader.peek_byte().unwrap() != FIELD_TERMINATOR {
            let entry = DirectoryEntry::read(reader, leader)?;
            entries.push(entry);
        }

        // Go past the field terminator
        reader.read_char()?;

        Ok(Directory { entries })
    }

    pub fn entries(&self) -> &Vec<DirectoryEntry> {
        &self.entries
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::leader::tests::ascii_ddr_leader;
    use crate::{Directory, Leader, ReadResult, Reader, FIELD_TERMINATOR};
    use std::io::{BufReader, Cursor};

    pub fn ascii_ddr_directory() -> ReadResult<(Leader, Directory)> {
        let leader = ascii_ddr_leader()?;

        let bytes = [
            "0000001630000000010004400163FRID0011400207FOID0007400321ATTF0006000395".as_bytes(),
            "NATF0006900450FFPC0008900524FFPT0008300613FSPC0008900696FSPT0009100785".as_bytes(),
            "VRID0008300876ATTV0005900959VRPC0007001018VRPT0007701088SGCC0005901165".as_bytes(),
            "SG2D0004601224SG3D0005101270ARCC0007801321AR2D0006001399EL2D0007401459".as_bytes(),
            "CT2D0004801533".as_bytes(),
            &[FIELD_TERMINATOR],
        ]
        .concat();
        let buffer = Cursor::new(bytes);
        let bufreader = BufReader::new(buffer);
        let mut reader = Reader::new(bufreader);

        let directory = Directory::read(&mut reader, &leader)?;

        Ok((leader, directory))
    }

    #[test]
    fn test_ddr_directory() {
        let target = ascii_ddr_directory();

        assert_eq!(target.is_ok(), true);

        let target = target.unwrap().1;
        assert_eq!(target.entries.len(), 21);
    }
}
