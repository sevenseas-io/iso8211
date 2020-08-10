use crate::{DDRLeader, DirectoryEntry, ReadResult, Reader, FIELD_TERMINATOR};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct Directory {
    entries: Vec<DirectoryEntry>,
}

impl Directory {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        leader: &DDRLeader,
    ) -> ReadResult<Directory> {
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
