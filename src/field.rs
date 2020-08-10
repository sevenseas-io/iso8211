use crate::{
    DDRLeader, DirectoryEntry, ReadError, ReadResult, Reader, FIELD_TERMINATOR, UNIT_TERMINATOR,
};

use std::io::{Read, Seek};

#[derive(Debug)]
pub struct FileControlField {
    tag_pairs: Vec<TagPair>,
}

#[derive(Debug)]
pub struct TagPair {
    parent: String,
    child: String,
}

#[derive(Debug)]
pub struct DataDescriptiveField {
    field_controls: FieldControls,
    field_name: String,
    array_descriptor: String,
    format_controls: String,
}

#[derive(Debug)]
pub struct FieldControls {
    data_structure: DataStructure,
    data_type: DataType,
    escape_sequence: LexicalLevel,
}

#[derive(Debug)]
enum DataStructure {
    SingleDataItem,
    LinearStructure,
    MultiDimensionalStructure,
    Unknown3,
}

#[derive(Debug)]
enum DataType {
    CharacterString,
    ImplicitPoint,
    Binary,
    Mixed,
}

#[derive(Debug)]
enum LexicalLevel {
    Level0,
    Level1,
    Level2,
    UnknownG,
}

impl FileControlField {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        leader: &DDRLeader,
        directory_entry: &DirectoryEntry,
    ) -> ReadResult<FileControlField> {
        let field_controls = reader.read_str(*leader.field_control_length() as usize)?;
        if field_controls != "0000;&   " {
            return Err(ReadError::ParseError(format!(
                "Invalid Field Controls: {}",
                field_controls
            )));
        }

        // we should have a unit terminator here
        if reader.read_u8()? != UNIT_TERMINATOR {
            return Err(ReadError::ParseError(String::from(
                "Did not find a unit terminator after the Field Controls",
            )));
        }

        // calculate the number of tag pairs
        let tag_length = *leader.entry_map().field_tag() as usize;
        let count = (*directory_entry.field_length() as usize - 11) / (2 * tag_length);
        let mut tag_pairs: Vec<TagPair> = Vec::with_capacity(count);
        for _ in 0..count {
            let parent = reader.read_str(tag_length)?;
            let child = reader.read_str(tag_length)?;
            tag_pairs.push(TagPair { parent, child });
        }

        // it should all end with a filed terminator here
        if reader.read_u8()? != FIELD_TERMINATOR {
            return Err(ReadError::ParseError(String::from(
                "Did not find a field terminator after the Field Controls",
            )));
        }

        Ok(FileControlField { tag_pairs })
    }
}

impl DataDescriptiveField {
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        leader: &DDRLeader,
        directory_entry: &DirectoryEntry,
    ) -> ReadResult<DataDescriptiveField> {
        // Data structure code
        let data_structure = reader.read_char()?;
        let data_structure = DataStructure::from_char(data_structure)?;

        // Data type code
        let data_type = reader.read_char()?;
        let data_type = DataType::from_char(data_type)?;

        // Auxiliary controls must be "00"
        let auxiliary_controls = reader.read_str(2)?;
        if auxiliary_controls != "00" {
            return Err(ReadError::ParseError(format!(
                "Invalid Auxiliary Controls: {}",
                auxiliary_controls
            )));
        }
        // Printable graphics must be ";&"
        let printable_graphics = reader.read_str(2)?;
        if printable_graphics != ";&" {
            return Err(ReadError::ParseError(format!(
                "Invalid Printable Graphics: {}",
                printable_graphics
            )));
        }
        // Truncated escape sequence
        let escape_sequence = reader.read_str(3)?;
        let escape_sequence = LexicalLevel::from_str(escape_sequence)?;
        let field_name = reader.read_str_ut()?;
        let array_descriptor = reader.read_str_ut()?;
        let format_controls = reader.read_str_ft()?;

        let field_controls = FieldControls {
            data_structure,
            data_type,
            escape_sequence,
        };

        Ok(DataDescriptiveField {
            field_controls,
            field_name,
            array_descriptor,
            format_controls,
        })
    }
}

impl DataStructure {
    fn from_char(value: char) -> ReadResult<DataStructure> {
        match value {
            '0' => Ok(DataStructure::SingleDataItem),
            '1' => Ok(DataStructure::LinearStructure),
            '2' => Ok(DataStructure::MultiDimensionalStructure),
            '3' => Ok(DataStructure::Unknown3),
            e => Err(ReadError::ParseError(format!(
                "Invalid Data Structure Code: {}",
                e
            ))),
        }
    }
}

impl DataType {
    fn from_char(value: char) -> ReadResult<DataType> {
        match value {
            '0' => Ok(DataType::CharacterString),
            '1' => Ok(DataType::ImplicitPoint),
            '5' => Ok(DataType::Binary),
            '6' => Ok(DataType::Mixed),
            e => Err(ReadError::ParseError(format!(
                "Invalid Data Type Code: {}",
                e
            ))),
        }
    }
}

impl LexicalLevel {
    fn from_str(value: String) -> ReadResult<LexicalLevel> {
        match value.as_ref() {
            "   " => Ok(LexicalLevel::Level0),
            "-A " => Ok(LexicalLevel::Level1),
            "%/A" => Ok(LexicalLevel::Level2),
            "%/G" => Ok(LexicalLevel::UnknownG),
            e => Err(ReadError::ParseError(format!(
                "Invalid Truncated Escape Sequence: {}",
                e
            ))),
        }
    }
}
