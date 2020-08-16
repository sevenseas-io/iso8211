use crate::{
    DirectoryEntry, Leader, ReadError, ReadResult, Reader, FIELD_TERMINATOR, UNIT_TERMINATOR,
};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct TagPair {
    parent: String,
    child: String,
}

#[derive(Debug)]
pub struct FileControlField {
    tag_pairs: Vec<TagPair>,
}

impl FileControlField {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        leader: &Leader,
        directory_entry: &DirectoryEntry,
    ) -> ReadResult<FileControlField> {
        let field_controls = reader.read_str(*leader.field_control_length() as usize)?;
        if field_controls != "0000;&   " {
            return Err(ReadError::ParseError(format!(
                "Invalid Field Controls: {}",
                field_controls
            )));
        }

        // we should have a unit terminator here
        if reader.read_u8()? != UNIT_TERMINATOR {
            return Err(ReadError::ParseError(String::from(
                "Did not find a unit terminator after the Field Controls",
            )));
        }

        // calculate the number of tag pairs
        let tag_length = *leader.entry_map().field_tag() as usize;
        let count = (*directory_entry.field_length() as usize - 11) / (2 * tag_length);
        let mut tag_pairs: Vec<TagPair> = Vec::with_capacity(count);
        for _ in 0..count {
            let parent = reader.read_str(tag_length)?;
            let child = reader.read_str(tag_length)?;
            tag_pairs.push(TagPair { parent, child });
        }

        // it should all end with a filed terminator here
        if reader.read_u8()? != FIELD_TERMINATOR {
            return Err(ReadError::ParseError(String::from(
                "Did not find a field terminator after the Field Controls",
            )));
        }

        Ok(FileControlField { tag_pairs })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::directory::tests::ascii_ddr_directory;
    use crate::{
        Directory, FileControlField, Leader, ReadResult, Reader, FIELD_TERMINATOR, UNIT_TERMINATOR,
    };
    use std::io::{BufReader, Cursor};

    pub fn ascii_file_control_field() -> ReadResult<(Leader, Directory, FileControlField)> {
        let data = ascii_ddr_directory()?;

        let bytes = [
            "0000;&   ".as_bytes(),
            &[UNIT_TERMINATOR],
            "0001FRIDFRIDFOIDFRIDATTFFRIDNATFFRIDFFPCFRIDFFPTFRIDFSPCFRID".as_bytes(),
            "FSPT0001VRIDVRIDATTVVRIDVRPCVRIDVRPTVRIDSGCCVRIDSG2DVRIDSG3DVRIDARCCA".as_bytes(),
            "RCCAR2DARCCEL2DARCCCT2D".as_bytes(),
            &[FIELD_TERMINATOR],
        ]
        .concat();
        let buffer = Cursor::new(bytes);
        let bufreader = BufReader::new(buffer);
        let mut reader = Reader::new(bufreader);
        let file_control_field =
            FileControlField::read(&mut reader, &data.0, &data.1.entries()[0])?;

        Ok((data.0, data.1, file_control_field))
    }

    #[test]
    fn test_file_control_field() {
        let target = ascii_file_control_field();

        // assert
        assert_eq!(target.is_ok(), true);

        let target = target.unwrap().2;
        assert_eq!(target.tag_pairs.len(), 19);
    }
}
