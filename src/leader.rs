use crate::{ReadError, ReadResult, Reader};

use std::io::{Read, Seek};

/*
RP      Len     Entry name                          Content
=================================================================================
0       5       Record length                       number of bytes in record
5       1       Interchange level                   "3"
6       1       Leader identifier                   "L"
7       1       In line code extension indicator    "E"
8       1       Version number                      "1"
9       1       Application indicator               SPACE
10      2       Field control length                "09"
12      5       Base address of field area          Start address of field area (number of bytes inleader and directory)
17      3       Extended character set indicator    " ! " (SPACE,!,SPACE)
20      4       Entry map                           (see table below)

RP      Sub-entry name                  Len     Content
=================================================================================
20      Size of field length field      1       Variable 1-9 (defined by encoder)
21      Size of field position field    1       Variable 1-9 (defined by encoder)
22      Reserved                        1       "0"
23      Size of field tag field         1       "4"
*/

/// The structure of the DR leader
#[derive(Debug, PartialEq)]
pub struct DDRLeader {
    /// Record Length
    record_length: u64,
    /// Interchange Level
    interchange_level: u8,
    /// Leader Identifier
    leader_identifier: char,
    /// In Line Code Extension Indicator
    code_extension: char,
    /// Version Number
    version_number: u8,
    /// Application Indicator
    application_indicator: char,
    /// Field Control Length
    field_control_length: u8,
    /// Base Address Of Field Area
    base_address: u64,
    /// Extended Character Set Indicator
    character_set: String,
    /// Entry Map
    entry_map: EntryMap,
}

impl DDRLeader {
    pub fn field_control_length(&self) -> &u8 {
        &self.field_control_length
    }

    pub fn entry_map(&self) -> &EntryMap {
        &self.entry_map
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EntryMap {
    /// Size Of Field Length Field
    field_length: u8,
    /// Size Of Field Position Field
    field_position: u8,
    /// Reserved
    reserved: char,
    /// Size Of Field Tag Field
    field_tag: u8,
}

impl EntryMap {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>) -> ReadResult<EntryMap> {
        let field_length = reader.read_u8_str(1)?;

        let field_position = reader.read_u8_str(1)?;

        let reserved = reader.read_char()?;

        let field_tag = reader.read_u8_str(1)?;

        Ok(EntryMap {
            field_length,
            field_position,
            reserved,
            field_tag,
        })
    }

    /// Size Of Field Length Field
    pub fn field_length(&self) -> &u8 {
        &self.field_length
    }

    /// Size Of Field Position Field
    pub fn field_position(&self) -> &u8 {
        &self.field_position
    }

    /// Size Of Field Tag Field
    pub fn field_tag(&self) -> &u8 {
        &self.field_tag
    }
}

impl DDRLeader {
    pub fn read<T: Read + Seek>(reader: &mut Reader<T>) -> ReadResult<DDRLeader> {
        let record_length = reader.read_u64_str(5)?;
        let interchange_level = reader.read_u8_str(1)?;
        if interchange_level != 3 {
            return Err(ReadError::ParseError(format!(
                "Invalid Interchange Level: {}",
                interchange_level
            )));
        }
        let leader_identifier = reader.read_char()?;
        if leader_identifier != 'L' {
            return Err(ReadError::ParseError(format!(
                "Invalid Leader Identifier: {}",
                leader_identifier
            )));
        }

        let code_extension = reader.read_char()?;
        if code_extension != 'E' {
            return Err(ReadError::ParseError(format!(
                "Invalid In Line Code Extension Indicator: {}",
                code_extension
            )));
        }

        let version_number = reader.read_u8_str(1)?;
        if version_number != 1 {
            return Err(ReadError::ParseError(format!(
                "Invalid Verison Number: {}",
                version_number
            )));
        }

        let application_indicator = reader.read_char()?;
        if application_indicator != ' ' {
            return Err(ReadError::ParseError(format!(
                "Invalid Application Indicator: {}",
                application_indicator
            )));
        }

        let field_control_length = reader.read_u8_str(2)?;
        if field_control_length != 09 {
            return Err(ReadError::ParseError(format!(
                "Invalid Field Control Length: {}",
                field_control_length
            )));
        }

        let base_address = reader.read_u64_str(5)?;

        let character_set = reader.read_str(3)?;
        if character_set != " ! " {
            return Err(ReadError::ParseError(format!(
                "Invalid Extended Character Set Indicator: {}",
                character_set
            )));
        }

        let entry_map = EntryMap::read(reader)?;

        Ok(DDRLeader {
            record_length,
            interchange_level,
            leader_identifier,
            code_extension,
            version_number,
            application_indicator,
            field_control_length,
            base_address,
            character_set,
            entry_map,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{DDRLeader, Reader};

    use std::io::{BufReader, Cursor};

    #[test]
    fn test_ddr_leader() {
        // arrange
        let bytes = "019003LE1 0900319 ! 5504".as_bytes();
        let buffer = Cursor::new(bytes);
        let bufreader = BufReader::new(buffer);
        let mut reader = Reader::new(bufreader);

        // act
        let target = DDRLeader::read(&mut reader);

        // assert
        assert_eq!(target.is_ok(), true);

        let target = target.unwrap();
        assert_eq!(target.record_length, 01900);
        assert_eq!(target.interchange_level, 3);
        assert_eq!(target.leader_identifier, 'L');
        assert_eq!(target.entry_map.field_length, 5);
        assert_eq!(target.entry_map.field_position, 5);
        assert_eq!(target.entry_map.reserved, '0');
        assert_eq!(target.entry_map.field_tag, 4);
    }
}
