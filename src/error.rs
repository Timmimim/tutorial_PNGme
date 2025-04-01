use std::error;
use std::fmt;
use std::panic::resume_unwind;

/*
    Create distinct Error categories depending on source within PNG / Chunk construction process.
*/

// Handle Errors occuring while instantiating ChunkTypes
#[derive(Debug)]
pub enum ChunkTypeError {
    // input bytes length smaller than the necessary 12 bytes for metadata
    InvalidBytes(String),
    //InvalidChunkType
    InvalidChunkType,
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkTypeError::InvalidBytes(bytes) => {
                write!(f, "Invalid input bytes provided: {}", bytes)
            },
            ChunkTypeError::InvalidChunkType => write!(f, "Invalid chunk type")
        }
    }
}

// Handle Errors occuring while instantiating Chunks
#[derive(Debug)]
pub enum ChunkError {
    // input bytes length smaller than the necessary 12 bytes for metadata
    InputTooSmall(u32),
    // invalid crc for chunk
    InvalidCrc(u32,u32),
    
    // InvalidChunkLength(String),
}


impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkError::InputTooSmall(actual_length) => {
                write!(f, "At least 12 bytes MUST be supplied to construct a chunk (was {})",actual_length)
            },
            ChunkError::InvalidCrc(expected, actual) => {
                write!(
                    f,
                    "Mismatching CRC IEEE/ISO-HDLC checksums: expected {}, but found {}",
                    expected, actual
                )
            }
        }
    }
}

// Handle Errors occuring while instantiating PNGs
#[derive(Debug)]
pub enum PNGError {
    // specified Chunk was not findable in PNG
    ChunkNotFound,
    // input bytes array too small to create valid PNG from
    TooSmall,
    // the input does not start with the necessary PNG header sequence
    InvalidSignature,
}

impl error::Error for PNGError {}

impl fmt::Display for PNGError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PNGError::ChunkNotFound => {
                write!(f, "Specified ChunkType cannot be found in this PNG")
            },
            PNGError::TooSmall => {
                write!(f, "Input is too small to create PNG / Chunks from")
            }
            PNGError::InvalidSignature => {
                write!(f, "Input begins with invalid set of bytes, mismatching necessary PNG signature header")
            }
        }
    }
}

// Handle Errors occuring while performing filesystem operations
#[derive(Debug)]
pub enum FsIoError {
    UnableToCreateFileError(String),
    UnableToWriteToOutputFileError(String),
    UnableToReadFileError(String),
}

impl fmt::Display for FsIoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FsIoError::UnableToCreateFileError(reason) => {
                write!(f, "Unable to create output file: {}", reason)
            },
            FsIoError::UnableToWriteToOutputFileError(reason) => {
                write!(f, "Unable to write to output file: {}", reason)
            },
            FsIoError::UnableToReadFileError(reason) => {
                write!(f, "Unable to read from source file: {}", reason)
            },
        }
    }
}