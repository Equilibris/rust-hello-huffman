#![feature(core_intrinsics)]
#![feature(vec_into_raw_parts)]
mod binary_tree;
mod huff_decoder;
mod huff_encoder;
mod types;
mod utils;
mod header;

pub use crate::huff_encoder::*;
use std::env;
use std::fs;

// Header Layout:
// Vers offB     Symbols-Size     Content size (max 4.294967296gb)
// 0000 0000 00000000000000000000 00000000000000000000000000000000
// sizes = 8, 16, 32, 64

// Tree symbols
// 00 = end node
// 01 = node followed by leaf
// 10 = node followed by node
// 11 = node followed by 2 leafs

fn main() -> std::io::Result<()> {
    // let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);

    let mut path = env::current_dir()?;
    path.push(
        // "huge_test.txt",
        "large_test.txt",
        // "medium_test.txt",
        // "standard_test.txt",
        // "small_test.txt",
    );
    let contents = fs::read_to_string(path)?;

    encoder(contents);

    Ok(())
}
