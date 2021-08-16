pub fn help() -> crate::cli::types::Out {
    println!(
        "
TOPIC:
Rust huffman CLI implementation

SHORT DESCRIPTION:
Enables the user to compress and decompress files encoded with rhm

SUB COMMANDS:
- help
	Displays a help message the likes of which you're reading now.
- e ARGS
- encode ARGS
	Encodes files and writes them to specified filenames.
- d ARGS
- decode ARGS
	Encodes files and writes them to specified filenames.
(sub commands are case-sensitive)

where:
	ARGS =
		a list of file names split by spaces and with output 
		specified after a ':' in the filenames or with
		default as FILENAME.nhm in decode mode and rhm in
		encode mode.

		EXAMPLES:
			hello.txt:output.txt
			hello.rhm (outputs hello.rhm.nhm)

EXAMPLES
	rhm encode hello.txt other.txt:out
	rhm decode hello.txt.rhm out:other.txt.nhm"
    );

    return Ok(());
}
