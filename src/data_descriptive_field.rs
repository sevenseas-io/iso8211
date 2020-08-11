use crate::{DirectoryEntry, ReadError, ReadResult, Reader};
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
    CharacterString = 0,
    ImplicitPoint = 1,
    ExplicitPoint = 2,
    Binary = 5,
    Mixed = 6,
}

impl DataType {
    fn from_char(value: char) -> ReadResult<DataType> {
        match value {
            '0' => Ok(DataType::CharacterString),
            '1' => Ok(DataType::ImplicitPoint),
            '2' => Ok(DataType::ExplicitPoint),
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
            "%/@" => Ok(LexicalLevel::Level2),
            //FIXME: Find out what this lexical level is
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
    pub fn read<T: Read + Seek>(
        reader: &mut Reader<T>,
        entry: &DirectoryEntry,
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

#[cfg(test)]
pub(crate) mod tests {
    use crate::directory::tests::ascii_ddr_directory;
    use crate::{
        DataDescriptiveField, Directory, ReadResult, Reader, FIELD_TERMINATOR, UNIT_TERMINATOR,
    };
    use std::io::{BufReader, Cursor};

    pub fn ascii_data_descriptive_field(
        index: usize,
        directory: &Directory,
    ) -> ReadResult<DataDescriptiveField> {
        let bytes = [
            [
                "0500;&   ISO 8211 Record Identifier".as_bytes(),
                &[UNIT_TERMINATOR, UNIT_TERMINATOR],
                "(b12)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "1600;&   Feature record identifier field".as_bytes(),
                &[UNIT_TERMINATOR],
                "RCNM!RCID!PRIM!GRUP!OBJL!RVER!RUIN".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b11,b14,2b11,2b12,b11)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "1600;&   Feature object identifier field".as_bytes(),
                &[UNIT_TERMINATOR],
                "AGEN!FIDN!FIDS".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b12,b14,b12)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2600;&-A Feature record attribute field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*ATTL!ATVL".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b12,A)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2600;&%/@Feature record national attribute field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*ATTL!ATVL".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b12,A)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "1600;&   Feature record to feature object pointer control field".as_bytes(),
                &[UNIT_TERMINATOR],
                "FFUI!FFIX!NFPT".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b11,2b12)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2600;&   Feature record to feature object pointer field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*LNAM!RIND!COMT".as_bytes(),
                &[UNIT_TERMINATOR],
                "(B(64),b11,A)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "1600;&   Feature record to spatial record pointer control field".as_bytes(),
                &[UNIT_TERMINATOR],
                "FSUI!FSIX!NSPT".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b11,2b12)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2600;&   Feature record to spatial record pointer field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*NAME!ORNT!USAG!MASK".as_bytes(),
                &[UNIT_TERMINATOR],
                "(B(40),3b11)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "1600;&   Vector record identifier field".as_bytes(),
                &[UNIT_TERMINATOR],
                "RCNM!RCID!RVER!RUIN".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b11,b14,b12,b11)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2600;&   Vector record attribute field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*ATTL!ATVL".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b12,A)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "1600;&   Vector record pointer control field".as_bytes(),
                &[UNIT_TERMINATOR],
                "VPUI!VPIX!NVPT".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b11,2b12)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2600;&   Vector record pointer field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*NAME!ORNT!USAG!TOPI!MASK".as_bytes(),
                &[UNIT_TERMINATOR],
                "(B(40),4b11)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "1600;&   Coordinate control field".as_bytes(),
                &[UNIT_TERMINATOR],
                "CCUI!CCIX!CCNC".as_bytes(),
                &[UNIT_TERMINATOR],
                "(b11,2b12)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2500;&   2-D Coordinate field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*YCOO!XCOO".as_bytes(),
                &[UNIT_TERMINATOR],
                "(2b24)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2500;&   3-D Coordinate field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*YCOO!XCOO!VE3D".as_bytes(),
                &[UNIT_TERMINATOR],
                "(3b24)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "1600;&   Arc/Curve definition field".as_bytes(),
                &[UNIT_TERMINATOR],
                "ATYP!SURF!ORDR!RESO!FPMF".as_bytes(),
                &[UNIT_TERMINATOR],
                "(3b11,2b14)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2500;&   Arc coordinate field".as_bytes(),
                &[UNIT_TERMINATOR],
                "STPT!CTPT!ENPT*YCOO!XCOO".as_bytes(),
                &[UNIT_TERMINATOR],
                "(2b24)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2500;&   Ellipse coordinate field".as_bytes(),
                &[UNIT_TERMINATOR],
                "STPT!CTPT!ENPT!CDPM!CDPR*YCOO!XCOO".as_bytes(),
                &[UNIT_TERMINATOR],
                "(2b24)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
            [
                "2500;&   Curve coordinate field".as_bytes(),
                &[UNIT_TERMINATOR],
                "*YCOO!XCOO".as_bytes(),
                &[UNIT_TERMINATOR],
                "(2b24)".as_bytes(),
                &[FIELD_TERMINATOR],
            ]
            .concat(),
        ];
        let buffer = Cursor::new(&bytes[index]);
        let bufreader = BufReader::new(buffer);
        let mut reader = Reader::new(bufreader);

        let entry = &directory.entries()[index];

        let data_descriptive_field = DataDescriptiveField::read(&mut reader, &entry)?;
        Ok(data_descriptive_field)
    }

    #[test]
    fn test_data_descriptive_fields() {
        let directory = ascii_ddr_directory().unwrap();

        for i in 0..20 {
            let target = ascii_data_descriptive_field(i, &directory.1);
            assert_eq!(target.is_ok(), true);
        }
    }
}
