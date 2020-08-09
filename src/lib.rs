mod document;
pub use document::Document;

mod directory;
mod leader;
mod field;

pub mod errors;
mod reader;

/// decimal value for ISO8211 field terminator
pub(crate) const FIELD_TERMINATOR: u8 = 0x1e;

/// decimal value for ISO8211 unit terminator
pub(crate) const UNIT_TERMINATOR: u8 = 0x1f;
