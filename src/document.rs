use crate::{DataDescriptiveRecord, DataRecord, ReadResult, Reader};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Document {
    data_descriptive_record: DataDescriptiveRecord,
    data_records: Vec<DataRecord>,
}

impl Document {
    pub fn read<P: AsRef<Path>>(path: P) -> ReadResult<Document> {
        let file = File::open(path.as_ref())?;
        let buffer = BufReader::new(Box::new(file));
        let mut reader = Reader::new(buffer);

        let data_descriptive_record = DataDescriptiveRecord::read(&mut reader)?;

        let data_record = DataRecord::read(&mut reader);
        let data_records = Vec::new();
        
        Ok(Document {
            data_descriptive_record: data_descriptive_record,
            data_records: data_records,
        })
    }
}
