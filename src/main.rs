#![feature(core_intrinsics)]
#![feature(vec_into_raw_parts)]
mod binary_tree;
mod huff_encoder;
mod types;
mod utils;

pub use crate::huff_encoder::*;
use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    // let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);

    let mut path = env::current_dir()?;
    path.push(
        // "huge_test.txt",
        // "large_test.txt",
        // "medium_test.txt",
        "standard_test.txt",
        // "small_test.txt",
    );
    let contents = fs::read_to_string(path)?;

    encoder(contents);

    Ok(())
}
