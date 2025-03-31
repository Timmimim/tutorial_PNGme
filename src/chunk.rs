use std::convert::TryFrom;
use std::fmt;
// use std::io::{BufReader, Bytes, Read};
use std::str;

use crc;
use anyhow::{anyhow, Result};

use crate::chunk_type::ChunkType;

// set up utility function for CRC
pub fn calculate_crc(chunk_data: &Vec<u8>) -> u32 {
    let crc_algorithm = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    let mut digest = crc_algorithm.digest();
    digest.update(&chunk_data);
    digest.finalize()
}

#[derive(Clone, Debug)]
pub struct Chunk {
    length: u32,
    chunk_type : ChunkType,
    data : Vec<u8>,
    crc: u32,
}

impl Chunk {
    // constructor
    pub fn new(chunk_type:ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let chunk_data: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc = calculate_crc(&chunk_data);

        Self {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    // getters 

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    // Returns data stored in this Chunk as a `String`. 
    // Will return Error if stored data is not valid UTF-8
    pub fn data_as_string(&self) -> Result<String>{
        match str::from_utf8(&self.data) {
            Ok(ok_string) => Ok(ok_string.to_string()),
            Err(err) => Err(anyhow!(err)),
        }
    }

    // Returns this Chunk as a byte sequence as described by PNG spec
    // The following data is included in this byte sequence in order: 
    //  1. Length of the data *(4 bytes)*
    //  2. Chunk type *(4 bytes)*
    //  3. The data itself *(`length` bytes)*
    //  4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }


}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;
    // create a Chunk from a list of bytes
    fn try_from(bytes: &[u8]) -> Result<Self> {
        // throw if input is shorter than necessary metadata length
        if bytes.len() < 12 {
            return Err(anyhow!(ChunkError::InputTooSmall));
        }

        // get first 4 bytes corresponding to chunks' data length
        let (data_length, bytes) = bytes.split_at(4);
        let length = u32::from_be_bytes(data_length.try_into()?);

        // get next 4 bytes corresponding to chunks' type
        let (chunk_type_bytes, bytes) = bytes.split_at(4);
        let chunk_type_bytes: [u8; 4] = chunk_type_bytes.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        if !chunk_type.is_valid() {
            return Err(anyhow!(ChunkError::InvalidChunkType));
        }

        // get chunks' data & crc from remaining bytes
        // length refers to chunks' data length
        let (data, bytes) = bytes.split_at(length as usize);
        let (crc,_) = bytes.split_at(4);

        let data: Vec<u8> = data.try_into()?;
        let crc = u32::from_be_bytes(crc.try_into()?);

        // calculate crc fresh from chunks' type & data
        let chunk_data: Vec<u8> = chunk_type.bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let actual_crc = calculate_crc(&chunk_data);
        let expected_crc = crc;
        if actual_crc != expected_crc {
            return Err(anyhow!(ChunkError::InvalidCrc(expected_crc, actual_crc)));
        }

        // all went well, return the fresh new Chunk
        Ok(
            Chunk { 
                length, 
                chunk_type, 
                data, 
                crc,
            }
        )

    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "length: {}, chunk type: {}, data: {:?}, crc{:?}",
            self.length, self.chunk_type, self.data, self.crc
        )
    }
}

#[derive(Debug)]
pub enum ChunkError {
    // input bytes length smaller than the necessary 12 bytes for metadata
    InputTooSmall,
    // invalid crc for chunk
    InvalidCrc(u32,u32),
    //InvalidChunkType
    InvalidChunkType,
}

// impl error::Error for ChunkError {}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkError::InputTooSmall => {
                write!(f, "At least 12 bytes MUST be supplied to construct a chunk")
            },
            ChunkError::InvalidCrc(expected, actual) => {
                write!(
                    f,
                    "Invalid CRC when constructing chunk. Expected {}, but found {}",
                    expected, actual
                )
            },
            ChunkError::InvalidChunkType => write!(f, "Invalid chunk type")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::{self, ChunkType};
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}