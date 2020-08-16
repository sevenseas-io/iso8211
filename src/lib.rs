pub mod ddr;

pub mod dr;

pub mod error;

mod ddf;
pub use ddf::DataDescriptiveFile;

mod directory;

mod directory_entry;

mod leader;

mod reader;
use reader::{ReadResult, Reader};

/// binary value for ISO8211 field terminator
const FIELD_TERMINATOR: u8 = 0x1e;

/// binary value for ISO8211 unit terminator
const UNIT_TERMINATOR: u8 = 0x1f;

/// binary value for null
const NULL: u8 = 0x00;
