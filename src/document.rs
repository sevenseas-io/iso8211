use crate::{DDRLeader, DataDescriptiveField, Directory, FileControlField, ReadResult, Reader};

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

        let ddr_leader = DDRLeader::read(&mut reader)?;

        let directory = Directory::read(&mut reader, &ddr_leader)?;

        let entries = directory.entries();

        //FIXME: what's this for???
        let _file_control_field = FileControlField::read(&mut reader, &ddr_leader, &entries[0]);

        let mut data_descriptive_fields: Vec<DataDescriptiveField> =
            Vec::with_capacity(entries.len() - 1);
        for i in 1..entries.len() {
            let ddf = DataDescriptiveField::read(&mut reader, &ddr_leader, &entries[i])?;
            data_descriptive_fields.push(ddf);
        }

        Ok(Document { ddr_leader })
    }

    pub fn ddr_leader(&self) -> &DDRLeader {
        &self.ddr_leader
    }
}
