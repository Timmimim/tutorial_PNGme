use std::fmt;
use std::str::FromStr;
use std::{convert::TryFrom, u8};

use anyhow::{anyhow, bail, Error, Result};

// use crate::chunk;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChunkType {
    data : String,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8;4] {
        self.data.as_bytes().try_into().unwrap()
    }

    pub fn is_critical(&self) -> bool {
        let first_byte: &[u8] = self.data[0..1].as_bytes();
        first_byte[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        let second_byte: &[u8] = self.data[1..2].as_bytes();
        second_byte[0].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let third_byte: &[u8] = self.data[2..3].as_bytes();
        third_byte[0].is_ascii_uppercase()
    }

    pub fn is_save_to_copy(&self) -> bool {
        let fourth_byte: &[u8] = self.data[0..1].as_bytes();
        fourth_byte[0].is_ascii_lowercase()
    }

    pub fn is_valid_byte(ascii_val_to_check: u8) -> bool {
        let mut valid_bytes: Vec<u8> = (b'A'..b'[').collect();
        let mut more_valid_bytes: Vec<u8> = (b'a'..b'{').collect();
        valid_bytes.append(&mut more_valid_bytes);
        valid_bytes.contains(&ascii_val_to_check)
        // b'A' <= ascii_val_to_check <= b'Z' || (ascii_val >= b'a' && ascii_val <= b'z')
    }

    pub fn is_valid(&self) -> bool {
        let data_as_bytes: &[u8] = self.data.as_bytes();
        let mut flag = 0;
        for ascii_val in data_as_bytes {
            if ChunkType::is_valid_byte(ascii_val.to_owned()) {
                continue;
            } else {
                flag = 1;
                break;
            }
        }
        if flag == 1 || !self.is_reserved_bit_valid() {
            false
        } else {
            true
        }
    }
}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(bytes: [u8;4]) -> Result<Self> {
        let string:String = String::from_utf8(bytes[..].into()).unwrap();
        let chunk = ChunkType { data: string };
        if chunk.is_valid() {
            Ok(chunk)
        } else {
            Err(anyhow!("The chunk is not valid"))
        }
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 4 {
            let chunk: String = s.to_string();
            let result = ChunkType { data: chunk };
            Ok(result)
        } else {
            Err(anyhow!("Expected 4 bytes, received {} instead", s.len()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }


}
