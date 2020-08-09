use crate::leader::DDRLeader;
use crate::reader::{Reader, ReadResult};
use crate::FIELD_TERMINATOR;
use std::io::{Read, Seek};

#[derive(Debug)]
pub(crate) struct Directory {
    pub(crate) entries: Vec<DirectoryEntry>,
}

#[derive(Debug)]
pub(crate) struct DirectoryEntry {
    pub(crate) field_length: u64,
    pub(crate) field_position: u64,
    pub(crate) field_tag: String,
}

impl Directory {
    pub(crate) fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        leader: &DDRLeader,
    ) -> ReadResult<Directory> {
        let mut entries: Vec<DirectoryEntry> = Vec::new();

        while reader.peek_byte().unwrap() != FIELD_TERMINATOR {
            let field_tag = reader.read_str(leader.entry_map.field_tag as usize)?;
            let field_length = reader.read_u64_str(leader.entry_map.field_length as usize)?;
            let field_position = reader.read_u64_str(leader.entry_map.field_position as usize)?;
            let entry = DirectoryEntry {
                field_length,
                field_position,
                field_tag,
            };
            entries.push(entry);
        }

        // Go past the field terminator
        reader.read_char()?;

        Ok(Directory { entries })
    }
}
