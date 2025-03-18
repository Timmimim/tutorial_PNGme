mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// consideration as suggested by [**Jordan**](https://github.com/jrdngr):
// use anyhow::{Context, Result,};  // may be used in future

fn main() {
    todo!()
}
