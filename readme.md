# Rust huffman CLI implementation

# Author

William Bj. Sørensen

# Short description

Enables the user to compress and decompress files encoded with rhm

# Sub commands:

- help
  - Displays a help message the likes of which you're reading now.
- e ARGS
- encode ARGS
  - Encodes files and writes them to specified filenames.
- d ARGS
- decode ARGS
  - Encodes files and writes them to specified filenames.

(sub commands are case-sensitive)

## Where:

- ### ARGS =

  a list of file names split by spaces and with output
  specified after a ':' in the filenames or with
  default as FILENAME.nhm in decode mode and rhm in
  encode mode.

  - ### Examples:
    ```
    hello.txt:output.txt
    hello.rhm (outputs hello.rhm.nhm)
    ```

# Examples:

```
rhm encode hello.txt other.txt:out
rhm decode hello.txt.rhm out:other.txt.nhm"
```
