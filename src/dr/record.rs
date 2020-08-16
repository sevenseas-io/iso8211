use crate::{dr::DataField, dr::Directory, dr::Leader, ReadResult, Reader};
use std::io::{Read, Seek};

pub struct DataRecord {
    leader: Leader,
    directory: Directory,
    data_fields: Vec<DataField>,
}

impl DataRecord {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>) -> ReadResult<DataRecord> {
        let leader = Leader::read_dr(reader)?;

        let directory = Directory::read(reader, &leader)?;

        let entries = directory.entries();
        let mut data_fields: Vec<DataField> = Vec::with_capacity(entries.len());
        for entry in entries {
            let df = DataField::read(reader, &entry)?;
            data_fields.push(df);
        }

        Ok(DataRecord {
            leader,
            directory,
            data_fields,
        })
    }
}
