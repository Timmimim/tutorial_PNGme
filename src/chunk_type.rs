use std::fmt;
use std::str::FromStr;
use std::{convert::TryFrom, u8};

use anyhow::{anyhow, Result};

// use crate::chunk;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    data : String,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8;4] {
        self.data.as_bytes().try_into().unwrap()
    }

    pub fn is_critical(&self) -> bool {
        // critical: beginning with capital char
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

    pub fn is_safe_to_copy(&self) -> bool {
        let fourth_byte: &[u8] = self.data[3..4].as_bytes();
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
        for ascii_val_to_check in bytes {
            if ChunkType::is_valid_byte(ascii_val_to_check) {
                continue;
            } else {
                return Err(anyhow!("Invalid input"));
            }
        }
        let string:String = String::from_utf8(bytes[..].into()).unwrap();
        let chunk = ChunkType { data: string };

        if chunk.is_valid() {
            Ok(chunk)
        } else {
            Err(anyhow!("The chunk is not valid"))
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 4 {
            let chars = s.as_bytes();
            for char in chars {
                if !ChunkType::is_valid_byte(*char) {
                    return Err(anyhow!("Invalid input String: {}", s));
                }
            }
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

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }
    
    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }
    
    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_invalid() {
        let chunk_a = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk_a.is_valid());

        let chunk_b = ChunkType::from_str("Ru1t");
        assert!(chunk_b.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_a: ChunkType = TryFrom::try_from([82,117,83,116]).unwrap();
        let chunk_type_b: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_a);
        let _are_chunks_equal = chunk_type_a == chunk_type_b;
    }

}
