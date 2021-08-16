use super::file_arg::FileArg;
use super::types::Out;
use crate::huff_decoder::decoder;

use std::env;
use std::fs;

pub fn decode(args: Vec<String>) -> Out {
    let mut err_string = String::new();

    for val in args
        .iter()
        .map(|val| {
            let val = FileArg::new(val.clone());

            std::thread::spawn(move || {
                let mut from = env::current_dir()?;

                from.push(val.get_read());

                let contents = fs::read(from)?;

                let mut to = env::current_dir()?;

                to.push(val.get_write(".nhm"));

                println!("{}", to.to_str().unwrap());

                let output = decoder(contents);

                fs::write(to, output)
            })
        })
        .map(|handle| handle.join())
    {
        // TODO: error handling
        match val {
            Ok(_) => {}
            Err(err) => {
                err_string.push_str(format!("{:?}", err).as_str());
            }
        }
    }

    if err_string.len() > 0 {
        return Err(err_string);
    }

    Ok(())
}
