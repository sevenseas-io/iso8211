use crate::{ReadError, ReadResult, Reader};
use std::io::{Read, Seek};

#[derive(Debug)]
pub enum DataStructure {
    SingleDataItem,
    LinearStructure,
    MultiDimensionalStructure,
    Unknown3,
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

#[derive(Debug)]
pub enum DataType {
    CharacterString,
    ImplicitPoint,
    Binary,
    Mixed,
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

#[derive(Debug)]
pub enum LexicalLevel {
    Level0,
    Level1,
    Level2,
    UnknownG,
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

#[derive(Debug)]
pub struct FieldControls {
    data_structure: DataStructure,
    data_type: DataType,
    escape_sequence: LexicalLevel,
}

#[derive(Debug)]
pub struct DataDescriptiveField {
    field_controls: FieldControls,
    field_name: String,
    array_descriptor: String,
    format_controls: String,
}

impl DataDescriptiveField {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>) -> ReadResult<DataDescriptiveField> {
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

#[cfg(test)]
pub(crate) mod tests {
    use crate::{DataDescriptiveField, ReadResult, Reader, FIELD_TERMINATOR, UNIT_TERMINATOR};
    use std::io::{BufReader, Cursor};

    pub fn ascii_data_descriptive_field1() -> ReadResult<DataDescriptiveField> {
        let bytes = [
            "0100;&   ISO 8211 Record Identifier".as_bytes(),
            &[UNIT_TERMINATOR, UNIT_TERMINATOR],
            " (I(5))".as_bytes(),
            &[FIELD_TERMINATOR],
        ]
        .concat();
        let buffer = Cursor::new(bytes);
        let bufreader = BufReader::new(buffer);
        let mut reader = Reader::new(bufreader);

        let data_descriptive_field = DataDescriptiveField::read(&mut reader)?;
        Ok(data_descriptive_field)
    }

    pub fn ascii_data_descriptive_field2() -> ReadResult<DataDescriptiveField> {
        let bytes = [
            "1600;&   Feature record identifier field".as_bytes(),
            &[UNIT_TERMINATOR],
            "RCNM!RCID!PRIM!GRUP!OBJL!RVER!RUIN".as_bytes(),
            &[UNIT_TERMINATOR],
            "(A(2),I(10),A(1),I(3),I(5),I(3),A(1))".as_bytes(),
            &[FIELD_TERMINATOR],
        ]
        .concat();
        let buffer = Cursor::new(bytes);
        let bufreader = BufReader::new(buffer);
        let mut reader = Reader::new(bufreader);

        let data_descriptive_field = DataDescriptiveField::read(&mut reader)?;
        Ok(data_descriptive_field)
    }

    #[test]
    fn test_data_descriptive_fields() {
        struct TestCase {
            data_descriptive_field: ReadResult<DataDescriptiveField>,
            field_name: String,
            array_descriptor: String,
            format_controls: String,
        }
        let test_cases = [
            TestCase {
                data_descriptive_field: ascii_data_descriptive_field1(),
                field_name: String::from("ISO 8211 Record Identifier"),
                array_descriptor: String::from(""),
                format_controls: String::from(" (I(5))"),
            },
            TestCase {
                data_descriptive_field: ascii_data_descriptive_field2(),
                field_name: String::from("Feature record identifier field"),
                array_descriptor: String::from("RCNM!RCID!PRIM!GRUP!OBJL!RVER!RUIN"),
                format_controls: String::from("(A(2),I(10),A(1),I(3),I(5),I(3),A(1))"),
            },
        ];

        for i in &test_cases {
            assert_eq!(i.data_descriptive_field.is_ok(), true);

            let target = i.data_descriptive_field.as_ref().unwrap();
            assert_eq!(target.field_name, i.field_name);
            assert_eq!(target.array_descriptor, i.array_descriptor);
            assert_eq!(target.format_controls, i.format_controls);
        }
    }
}
