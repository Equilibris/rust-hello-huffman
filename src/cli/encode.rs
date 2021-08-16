use super::file_arg::FileArg;
use super::types::Out;
use crate::huff_encoder::encoder;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn encode(args: Vec<String>) -> Out {
    for val in args
        .iter()
        .map(|val| {
            let val = FileArg::new(val.clone());

            std::thread::spawn(move || {
                let mut from = env::current_dir()?;

                from.push(val.get_read());

                let contents = fs::read_to_string(from)?;

                let mut to = env::current_dir()?;

                to.push(val.get_write(".rhm"));

                println!("{}", to.to_str().unwrap());

                let output = encoder(contents);

                File::create(to)?.write_all(&output)
            })
        })
        .map(|handle| handle.join())
    {
        // TODO: error handling
        match val {
            Ok(_) => {}
            Err(err) => {
                return Err(format!("{:?}", err));
            }
        }
    }

    Ok(())
}
