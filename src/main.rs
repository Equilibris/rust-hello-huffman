#![feature(core_intrinsics)]
#![feature(vec_into_raw_parts)]
mod binary_tree;
mod cli;
mod header;
mod huff_decoder;
mod huff_encoder;
mod types;
mod utils;

pub use crate::huff_encoder::*;
use std::env::args;

fn main() -> crate::cli::types::Out {
    let args: Vec<String> = args().collect();

    crate::cli::entry(args)
}
