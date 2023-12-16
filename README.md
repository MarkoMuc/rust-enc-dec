# Rust File Encoder and Decoder 

## :warning: IMPORTANT :warning: This is not a secure algorithm! Do not use this on actual sensitive data.

This is an incredibly simple encoder and decoder written in Rust. It should not actually be used for any real sensitive data.

## Usage

- Run ``cargo run --release`` to build.
- It takes one argument, and that is the file it should encode and decode.

## How it works

- The encoding :
    - Generates a seed.
    - Uses a seeded generator to generate a random number in the range of 1 to 4.
    - Takes the random number and swaps that amount of bytes.
    - Does this until all bytes are covered; if needed, adds 0x0s.
    - At the end of the file, adds 0x4 ASCII and the seed that was used.

- The decoding:
    - Checks if the file is encoded.
    - Reads the seed. Removes the seed and 0x4 from the end of the file.
    - Swaps the bytes the same way as before.
    - Removes the added 0x0s.
