use crate::{dr::DirectoryEntry, ReadResult, Reader};
use std::io::{Read, Seek};

pub struct DataField {}

impl DataField {
    pub fn read<T: Read + Seek>(
        _reader: &mut Reader<T>,
        _entry: &DirectoryEntry,
    ) -> ReadResult<DataField> {
        Ok(DataField {})
    }
}
