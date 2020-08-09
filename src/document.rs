use crate::directory::Directory;
use crate::field::{DataDescriptiveField, FieldControlField};
use crate::leader::DDRLeader;
use crate::reader::{ReadResult, Reader};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct Document {
    ddr_leader: DDRLeader,
}

impl Document {
    pub fn read<P: AsRef<Path>>(path: P) -> ReadResult<Document> {
        let file = File::open(path.as_ref())?;
        let buffer = BufReader::new(Box::new(file));
        let mut reader = Reader::new(buffer);

        let ddr_leader = DDRLeader::read_ddr_leader(&mut reader)?;

        let ddr_directory = Directory::read(&mut reader, &ddr_leader)?;

        let _file_control_field =
            FieldControlField::read(&mut reader, &ddr_leader, &ddr_directory.entries[0]);

        let mut data_descriptive_fields: Vec<DataDescriptiveField> =
            Vec::with_capacity(ddr_directory.entries.len() - 1);
        for i in 1..ddr_directory.entries.len() {
            let ddf =
                DataDescriptiveField::read(&mut reader, &ddr_leader, &ddr_directory.entries[i])?;
            data_descriptive_fields.push(ddf);
        }

        Ok(Document { ddr_leader })
    }

    pub fn ddr_leader(&self) -> &DDRLeader{
        &self.ddr_leader
    }
}
