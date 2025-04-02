use std::env;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(subcommand)]
    pub command : PngMeArgs
}

#[derive(Debug, Parser, Clone)]
#[command(name="pngme", about="Hide secret messages inside valid PNG files")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

/// add a secret message at the end of a PNG file, after EOF chunk
#[derive(Debug, Parser, PartialEq, Clone)]
#[command(version, about, long_about=None)]
pub struct EncodeArgs {
    // file path for PNG file
    #[arg(short='f', long, value_name="PNG_PATH", value_hint=clap::ValueHint::DirPath)]
    pub input_path : PathBuf,
    // type of the new messages´ chunk
    #[arg(short='c', long, value_name="CHUNK_TYPE")]
    pub chunk_type : String,
    // message to encode in PNG file
    #[arg(short='m', long, value_name="MESSAGE")]
    pub message : String,
    // optional: new file path for output PNG
    #[arg(short='o', long, value_name="NEW_FILEPATH", value_hint=clap::ValueHint::DirPath)]
    pub output_file : Option<PathBuf>,
}

/// show the hidden message(s) in a PNG file
#[derive(Debug, Parser, PartialEq, Clone)]
#[command(version, about, long_about=None)]
pub struct DecodeArgs {
    // file path of PNG file
    #[arg(short='f', long, value_name="PNG_PATH", value_hint=clap::ValueHint::DirPath)]
    pub input_path: PathBuf,
    // type of the new messages´ chunk
    #[arg(short='c', long, value_name="CHUNK_TYPE")]
    pub chunk_type : String,
    // iterate ALL entries and decode all matching chunk_types
    #[arg(short='m', long, action)]
    pub multiple_chunks : bool,
}

/// remove the first chunk matching matching the specified chunk type
#[derive(Debug, Parser, PartialEq, Clone)]
#[command(version, about, long_about=None)]
pub struct RemoveArgs {
    // type of the (message) chunk to remove
    #[arg(short='c', long, value_name="CHUNK_TYPE")]
    pub chunk_type : String,
    // file path for PNG file
    #[arg(short='f', long, value_name="PNG_PATH", value_hint=clap::ValueHint::DirPath)]
    pub input_path: PathBuf,
    // optional: new file path for output PNG
    #[arg(short='o', long, value_name="NEW_FILEPATH", value_hint=clap::ValueHint::DirPath)]
    pub output_file : Option<PathBuf>,
    // iterate ALL entries and remove all matching chunk_types
    #[arg(short='m', long, action)]
    pub multiple_chunks : bool,
}

/// print out the (raw) PNG file chunk by chunk 
#[derive(Debug, Parser, PartialEq, Clone)]
#[command(version, about, long_about=None)]
pub struct PrintArgs {
    // file path for PNG file
    #[arg(short='f', long, value_name="PNG_PATH", value_hint=clap::ValueHint::DirPath)]
    pub input_path: PathBuf,
}