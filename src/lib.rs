mod document;
pub use document::Document;

mod data_descriptive_field;
pub use data_descriptive_field::{
    DataDescriptiveField, DataStructure, DataType, FieldControls, LexicalLevel,
};

mod data_descriptive_record;
pub use data_descriptive_record::DataDescriptiveRecord;

mod data_field;
pub use data_field::DataField;

mod data_record;
pub use data_record::DataRecord;

mod directory;
pub use directory::Directory;

mod directory_entry;
pub use directory_entry::DirectoryEntry;

mod errors;
pub use errors::ReadError;

mod file_control_field;
pub use file_control_field::{FileControlField, TagPair};

mod leader;
pub use leader::Leader;

mod reader;
pub use reader::{ReadResult, Reader};

/// binary value for ISO8211 field terminator
const FIELD_TERMINATOR: u8 = 0x1e;

/// binary value for ISO8211 unit terminator
const UNIT_TERMINATOR: u8 = 0x1f;

/// binary value for null
const NULL: u8 = 0x00;
