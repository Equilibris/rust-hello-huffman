mod decode;
mod encode;
mod file_arg;
mod help;
pub mod types;

pub fn entry(args: Vec<String>) -> crate::cli::types::Out {
    if args.len() < 2 {
        return Err(
            "Cannot omit sub command, use sub command help to view sub commands.".to_string(),
        );
    }

    let sub_command = &args[1];

    // TODO: multi threading
    if *sub_command == String::from("encode") || *sub_command == String::from("e") {
        return encode::encode(args[2..].to_vec());
    } else if *sub_command == String::from("decode") || *sub_command == String::from("d") {
        return decode::decode(args[2..].to_vec());
    } else if *sub_command == String::from("help") {
        return crate::cli::help::help();
    }

    Err(format!("{} is not a valid sub command, please use sub command help to view available sub commands.", sub_command))
}
