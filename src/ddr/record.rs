use crate::{
    ddr::{DataDescriptiveField, Directory, FileControlField, Leader},
    ReadResult, Reader,
};
use std::io::{Read, Seek};

#[derive(Debug)]
pub struct DataDescriptiveRecord {
    leader: Leader,
    directory: Directory,
    file_control_field: FileControlField,
    data_descriptive_fields: Vec<DataDescriptiveField>,
}

impl DataDescriptiveRecord {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>) -> ReadResult<DataDescriptiveRecord> {
        let leader = Leader::read_ddr(reader)?;

        let directory = Directory::read(reader, &leader)?;

        let entries = directory.entries();

        let file_control_field = FileControlField::read(reader, &leader, &entries[0])?;

        let mut data_descriptive_fields: Vec<DataDescriptiveField> =
            Vec::with_capacity(entries.len() - 1);
        for i in 1..entries.len() {
            let ddf = DataDescriptiveField::read(reader, &entries[i])?;
            data_descriptive_fields.push(ddf);
        }

        Ok(DataDescriptiveRecord {
            leader,
            directory,
            file_control_field,
            data_descriptive_fields,
        })
    }

    pub fn leader(&self) -> &Leader {
        &self.leader
    }
}
