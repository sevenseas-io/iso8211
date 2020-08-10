use crate::{DDRLeader, ReadResult, Reader};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct DirectoryEntry {
    field_length: u64,
    field_position: u64,
    field_tag: String,
}

impl DirectoryEntry {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        leader: &DDRLeader,
    ) -> ReadResult<DirectoryEntry> {
        let entry_map = leader.entry_map();
        let field_tag = reader.read_str(*entry_map.field_tag() as usize)?;
        let field_length = reader.read_u64_str(*entry_map.field_length() as usize)?;
        let field_position = reader.read_u64_str(*entry_map.field_position() as usize)?;
        Ok(DirectoryEntry {
            field_length,
            field_position,
            field_tag,
        })
    }

    pub fn field_length(&self) -> &u64 {
        &self.field_length
    }

    pub fn field_position(&self) -> &u64 {
        &self.field_position
    }

    pub fn field_tag(&self) -> &String {
        &self.field_tag
    }
}
