mod document;
pub use document::Document;

mod directory;
use directory::Directory;

mod directory_entry;
use directory_entry::DirectoryEntry;

mod field;
use field::{DataDescriptiveField, FieldControls, FileControlField, TagPair};

mod leader;
use leader::DDRLeader;

mod errors;
use errors::ReadError;

mod reader;
use reader::{ReadResult, Reader};

/// decimal value for ISO8211 field terminator
const FIELD_TERMINATOR: u8 = 0x1e;

/// decimal value for ISO8211 unit terminator
const UNIT_TERMINATOR: u8 = 0x1f;
