/* 
    If you're writing a function that reads from a file, 
    there's a nice way to accept the file's path as a parameter 
    using the AsRef trait. 
    Your function signature will look something like 
    #   fn from_file<P: AsRef<Path>>(path: P). 
*/
use std::fs;
use std::{convert::TryFrom, str::FromStr};

use crate::{args, chunk_type};
pub use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs, PngMeArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    error::FsIoError,
    png::Png,
};


use anyhow::{anyhow, Result};
use clap::Subcommand;

/* 
    Steganography is the practice of concealing information within another message or physical object to avoid detection.
    Steganography can be used to hide virtually any type of digital content, including text, image, video, or audio content.
    That hidden data is then extracted at its destination.

    A common approach is LSB (least significant bit) steganography. In the context of PNG images, instead of singular bits, 
    the least significant byte in the PNG file is changed to store information, undetectable to the human eye.
    Computers, however, can extract the information easily.
*/

/// encode a message into a PNG file and save the results, optionally to a new file
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::read_file(args.input_path.clone())?;
    // make the output either a specified optional path, or the original input path
    let output_path = &args.output_file.unwrap_or_else(|| args.input_path.clone());
    
    /*  
        Chunk::append will add the secret message at the very end of the PNG file, even after the IEND chunk.
        This roughly equates to an interpretation of "least significant bit", with normal PNG decoders not picking up the message.
        The contained image is not altered.
    */
    let chunk = Chunk::new(
        ChunkType::from_str(args.chunk_type.as_str()).unwrap(),
        args.message.as_bytes().to_vec(),
    );
    png.append_chunk(chunk);
    png.write_file(output_path)?;
    Ok(())
}

/// search for hidden message in a PNG file; print the message if it exists
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::read_file(args.input_path)?;
    let chunk = png.chunk_by_type(args.chunk_type.as_str());
    if let Some(retrieved_chunk) = chunk {
        println!("{}", retrieved_chunk);
        println!("Decodes as: {}", retrieved_chunk.data_as_string().unwrap());
    }
    Ok(())
}

/// remove a chunk from a PNG file and save the resulting PNG
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::read_file(args.input_path.clone())?;
    let removed_chunk = png.remove_chunk(&args.chunk_type)?;
    // make the output either a specified optional path, or the original input path
    let output_path = &args.output_file.unwrap_or_else(|| args.input_path.clone());
    png.write_file(output_path)?;
    Ok(())
}

/// print all chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::read_file(args.input_path)?;
    for chunk in png.chunks() {
        println!("Chunk: {}", chunk);
    }
    Ok(())
}

///Run the above program based on specified subcommand
pub fn run(subcommand: PngMeArgs) -> Result<()> {
    match subcommand {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print_chunks(args),
    }
}