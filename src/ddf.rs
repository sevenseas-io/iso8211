use crate::{ddr::DataDescriptiveRecord, dr::DataRecord, ReadResult, Reader};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct DataDescriptiveFile {
    data_descriptive_record: DataDescriptiveRecord,
    data_records: Vec<DataRecord>,
}

impl DataDescriptiveFile {
    pub fn read<P: AsRef<Path>>(path: P) -> ReadResult<DataDescriptiveFile> {
        let file = File::open(path.as_ref())?;
        let buffer = BufReader::new(Box::new(file));
        let mut reader = Reader::new(buffer);

        let data_descriptive_record = DataDescriptiveRecord::read(&mut reader)?;

        let mut data_records = Vec::new();
        while !reader.is_eof()? {
            let data_record = DataRecord::read(&mut reader)?;
            data_records.push(data_record);
        }
        Ok(DataDescriptiveFile {
            data_descriptive_record,
            data_records,
        })
    }

    pub fn data_descriptive_record(&self) -> &DataDescriptiveRecord {
        &self.data_descriptive_record
    }

    pub fn data_records(&self) -> &Vec<DataRecord> {
        &self.data_records
    }
}
