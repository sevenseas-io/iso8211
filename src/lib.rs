mod document;
pub use document::Document;

mod directory;
pub use directory::Directory;

mod directory_entry;
pub use directory_entry::DirectoryEntry;

mod field;
pub use field::{DataDescriptiveField, FieldControls, FileControlField, TagPair};

mod leader;
pub use leader::DDRLeader;

mod errors;
pub use errors::ReadError;

mod reader;
pub use reader::{ReadResult, Reader};

/// decimal value for ISO8211 field terminator
const FIELD_TERMINATOR: u8 = 0x1e;

/// decimal value for ISO8211 unit terminator
const UNIT_TERMINATOR: u8 = 0x1f;
