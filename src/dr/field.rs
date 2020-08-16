use crate::{dr::DirectoryEntry, error::ReadError, ReadResult, Reader};
use std::io::{Read, Seek};

pub struct DataField {}

impl DataField {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        entry: &DirectoryEntry,
    ) -> ReadResult<DataField> {
        Ok(DataField {})
    }
}
