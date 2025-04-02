mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod error;

use anyhow::Result;
use args::Commands;

use clap::{builder::styling::Color, Parser};

// consideration as suggested by [**Jordan**](https://github.com/jrdngr):
// use anyhow::{Context, Result,};  // may be used in future

fn main() -> Result<()>{
    // use the parse trait implemented by the Commands struct
    let args = Commands::parse();
    commands::run(args.command)?;
    Ok(())
}
